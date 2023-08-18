use crate::errors::RsError;
use crate::events::ApiEventTrigger;
use crate::GlobalAppState;
#[allow(dead_code)]
#[taurpc::procedures(export_to = "../src/lib/bindings.ts")]
pub trait RootApi {
    async fn test_error(msg: String) -> Result<bool, RsError>;
    async fn perm_ctlr_connect(port: String) -> Result<bool, RsError>;
    async fn set_permeability(flow: f64, pressure: f64) -> ();
}

#[derive(Clone)]
pub(crate) struct RootApiImpl {
    pub state: GlobalAppState,
}

#[taurpc::resolvers]
impl RootApi for RootApiImpl {
    async fn test_error(self, msg: String) -> Result<bool, RsError> {
        Err(RsError::Other(msg))
    }

    async fn perm_ctlr_connect(self, port: String) -> Result<bool, RsError> {
        println!("Listen to {:?}", port);
        Ok(true)
    }

    async fn set_permeability(self, flow: f64, pressure: f64) -> () {
        println!("set_permeability, {:?}, {:?}, {:?}", flow, pressure, self.state.lock().await.app_id);
        let state = self.state.lock().await;
        let app_handle = state.app_handle.as_ref().unwrap();
        let trigger = ApiEventTrigger::new(app_handle.clone());
        let ev = format!("flow rate:{:?}, pressure:{:?}", flow, pressure);
        let _ = trigger.ev(ev);
    }
}
