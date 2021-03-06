use juniper::{ FieldResult, EmptySubscription };

use super::user::{ User };
use super::machine::{ self, Machine };
use crate::Context;

pub struct MyNamespace;

#[graphql_object(
    Context = Context,
)]
impl MyNamespace {
    async fn machines(context: &Context, slug: Option<String>) -> FieldResult<Vec<Machine>> {
        Ok(machine::my_machines(context, slug).await?)
    }
}

pub struct Query;

#[graphql_object(
    Context = Context,
)]
impl Query {
    // fn apiVersion() -> &str {
    //     "1.0"
    // }

    fn my() -> FieldResult<MyNamespace> {
        Ok(MyNamespace)
    }

    async fn current_user(context: &Context) -> FieldResult<Option<User>> {
        Ok(context.user.clone())
    }

    async fn ice_servers(context: &Context) -> FieldResult<Vec<crate::ice_server::IceServer>> {
        Ok((*context.ice_servers.read().await).clone())
    }

    // fn is_authenticated_for(context: &Context, machine_id: String) -> FieldResult<bool> {
    //     Ok(context.user.is_some())
    // }
}

pub struct Mutation;

#[graphql_object(
    Context = Context,
)]
impl Mutation {
    async fn create_machine(
        context: &Context,
        input: machine::CreateMachineInput
    ) -> FieldResult<Machine> {
        Ok(machine::create_machine(context, input).await?)
    }

    async fn set_machine_name(context: &Context, input: machine::SetMachineName) -> FieldResult<Machine> {
        Ok(machine::set_machine_name(context, input).await?)
    }

    async fn remove_machine(context: &Context, machine_id: String) -> FieldResult<Option<bool>> {
        Ok(machine::remove_machine(context, machine_id).await?)
    }
}

// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
