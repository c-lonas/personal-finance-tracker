use yew::{prelude::*, Context};
use yew::events::InputEvent;
use yew::classes;
use gloo_timers::callback::Timeout;
use shared::income::IncomeCreate;
use wasm_bindgen::JsCast;
use log::info;
// use web_logger;

use crate::api_error::ApiError;

pub struct AddIncomeForm {
    user_income: IncomeCreate,
    submission_status: SubmissionStatus,
    submission_task: Option<Timeout>,
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_close: Callback<MouseEvent>,
    pub selected_user_id: Option<u32>,
    pub user_name: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            on_close: Callback::noop(),
            selected_user_id: Some(0),
            user_name: Some("Unknown".to_owned())
   
        }
    }
}

pub enum SubmissionStatus {
    Idle,
    Success,
}

pub enum Msg {
    UpdateIncome(IncomeCreate),
    SubmitForm,
    SubmissionStatus(SubmissionStatus),
    StartSubmissionReset,
    ResetSubmission,
    CloseForm,
    NoOp
}


async fn submit_income(user_id: u32, income: IncomeCreate) -> Result<(), ApiError> {
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:8000/api/income")
        .json(&IncomeCreate { user_id, ..income })
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::HttpStatus(response.status()))
    }
}


impl AddIncomeForm {
    fn start_submission_reset(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let handle = Timeout::new(1_000, move || {
            link.send_message(Msg::ResetSubmission);
        });
        self.submission_task = Some(handle);
    }
}

impl Component for AddIncomeForm {
    type Message = Msg;
    type Properties = Props;

    

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            // props: Props::default(),
            user_income: IncomeCreate {
                user_id: match ctx.props().selected_user_id {
                    Some(id) => id,
                    None => 0 
                },
                ..IncomeCreate::default()
            },
            submission_status: SubmissionStatus::Idle,
            submission_task: None,
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateIncome(new_income) => {
                self.user_income = new_income;
                true // Re-render the component
            }
            Msg::SubmitForm => {
                if self.user_income.name.is_empty() || self.user_income.amount == 0 {
                    log::warn!("Invalid input: Name or Amount is empty or incorrect");
                    return false;
                }
                let selected_user_id = ctx.props().selected_user_id;

                let income = self.user_income.clone();
                let link = ctx.link().clone();
                ctx.link().send_future(async move {
                    let user_id = match selected_user_id {
                        Some(id) => id,
                        None => 0,
                    };
                    
                    match submit_income(user_id, income).await {
                        Ok(_) => {
                            link.send_message(Msg::UpdateIncome(IncomeCreate::default()));
                            link.send_message(Msg::SubmissionStatus(SubmissionStatus::Success));
                            // link.send_message(Msg::ResetSubmission);
                            link.send_message(Msg::StartSubmissionReset);
                     
                        
                        }
                        Err(err) => {
                            log::error!("Error submitting income: {:?}", err);
                            link.send_message(Msg::NoOp); // to do: add error handling
                        }
                    }
                    
                    Msg::NoOp
                });
                ctx.link().send_message(Msg::NoOp); // Required to process future
                true // Re-render the component
            }
            Msg::SubmissionStatus(status) => {
                self.submission_status = status;
                true
            }
            Msg::StartSubmissionReset => {
                self.start_submission_reset(ctx);
                false
            }
            Msg::ResetSubmission => {
                self.submission_status = SubmissionStatus::Idle;
                true // Re-render the component
            }
            Msg::CloseForm => {
                ctx.props().on_close.emit(MouseEvent::new("click").unwrap()); // to do: add error handling
                true
            }

            Msg::NoOp => false, // Don't re-render the component
        }
    }


    fn changed(&mut self, ctx: &Context<Self>, props: &Self::Properties) -> bool {
        if self.user_income.user_id != props.selected_user_id.unwrap_or(0) {
            self.user_income.user_id = props.selected_user_id.unwrap_or(0);
            true
        } else if ctx.props().user_name != props.user_name {
            false
        } else {
            false
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {

        let success_class = match self.submission_status {
            SubmissionStatus::Idle => "",
            SubmissionStatus::Success => "success",
        };

        let user_id = match ctx.props().selected_user_id {
            Some(id) => id,
            None => 0, 
        };


        let unknown_user = String::from("Unknown");
        let user_name = ctx.props().user_name.as_ref().unwrap_or(&unknown_user);


        // clone values for the Name field
        let user_income_name1 = self.user_income.name.clone();
        let user_income_description1 = self.user_income.description.clone();
        let user_income_amount1 = self.user_income.amount;

        // clone values for the Description field
        let user_income_name2 = self.user_income.name.clone();
        let user_income_description2 = self.user_income.description.clone();
        let user_income_amount2 = self.user_income.amount;

        // clone values for the Amount field
        let user_income_name3 = self.user_income.name.clone();
        let user_income_description3 = self.user_income.description.clone();
        let user_income_amount3 = self.user_income.amount;

        html! {
            <div class= {classes!("card-main", success_class)} >
                <h2>{ "Add Income for:" }</h2>
                <h2>{ format!("Demo User: {} | Id: {}", user_name, user_id) }</h2>
                <input
                    placeholder="Income Name"
                    value={user_income_name1.clone()}
                    oninput={ctx.link().callback(move |event: InputEvent| {
                        let name_clone = user_income_name1.clone();
                        let description_clone = user_income_description1.clone();
                        let amount_clone = user_income_amount1;

                        if let Some(target) = event.target() {
                            if let Ok(input_element) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                let name = input_element.value();
                                let msg = Msg::UpdateIncome(IncomeCreate { user_id, name, ..IncomeCreate {
                                    user_id,
                                    name: name_clone,
                                    description: description_clone,
                                    amount: amount_clone,
                                }});
                                return msg;
                            }
                        }
                        Msg::NoOp
                    })}
                />
                <input
                    placeholder="Income Description"
                    value={self.user_income.description.clone().unwrap_or_default()}
                    oninput={ctx.link().callback(move |event: InputEvent| {
                        let name_clone = user_income_name2.clone();
                        let description_clone = user_income_description2.clone();
                        let amount_clone = user_income_amount2;

                        if let Some(target) = event.target() {
                            if let Ok(input_element) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                let description = input_element.value();
                                let msg = Msg::UpdateIncome(IncomeCreate { user_id, description: Some(description), ..IncomeCreate {
                                    user_id,
                                    name: name_clone,
                                    description: description_clone,
                                    amount: amount_clone,
                                }});
                                return msg;
                            }
                        }
                        Msg::NoOp
                    })}
                />
                <input
                    placeholder="Amount"
                    value={user_income_amount3.to_string()}
                    oninput={ctx.link().callback(move |event: InputEvent| {
                        let name_clone = user_income_name3.clone();
                        let description_clone = user_income_description3.clone();
                        let amount_clone = user_income_amount3;

                        if let Some(target) = event.target() {
                            if let Ok(input_element) = target.dyn_into::<web_sys::HtmlInputElement>() {
                                if let Ok(amount) = input_element.value().parse::<u32>() {
                                    let msg = Msg::UpdateIncome(IncomeCreate { user_id, amount, ..IncomeCreate {
                                        user_id,
                                        name: name_clone,
                                        description: description_clone,
                                        amount: amount_clone,
                                    }});
                                    return msg;
                                } else {
                                    return Msg::NoOp;
                                }
                            }
                        }
                        Msg::NoOp
                    })}
                />
                    <button
                        onclick={ctx.link().callback(|_| {
                            info!("Submit button clicked");
                            Msg::SubmitForm
                        })}
                    >
                        { "Submit" }    
                    </button>
                    <button
                        onclick={ctx.link().callback(|_| {
                            info!("Close button clicked");
                            Msg::CloseForm
                        })}
                    >
                        { "X" }
                    </button>

                </div>
            }
    }

}
