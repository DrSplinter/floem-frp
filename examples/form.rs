// use floem_frp::{
//     component::{button, dynamic, label, text_input, ArcComponent, ComponentTuple},
//     Component,
// };
// use frp::{
//     signal::{self, FrpSignalExt, Reactive, Value},
//     stream::{events, Events, FrpStreamExt},
// };
// use futures::{stream, StreamExt};
// use futures_signals::{
//     map_ref,
//     signal::{Signal, SignalExt},
// };

// #[derive(Clone)]
// struct EmailForm {
//     validated_email: Value<Reactive, Option<String>>,
//     components: ArcComponent,
// }

// fn validate_email(email: String) -> Option<String> {
//     if email.contains('@') {
//         Some(email)
//     } else {
//         None
//     }
// }

// fn error_message(email: Option<String>) -> String {
//     match email {
//         Some(_) => "".to_string(),
//         None => "Invalid email".to_string(),
//     }
// }

// impl EmailForm {
//     fn new(email_label: String) -> Self {
//         let email_label = label(signal::always_pending(email_label));
//         let email_input = text_input("", stream::pending::<String>());
//         let email = email_input.text().materialize();
//         let validated_email = email.signal_cloned().map(validate_email).materialize();
//         let validation_error_label = label(validated_email.signal_cloned().map(error_message));
//         let components = (email_label, email_input, validation_error_label)
//             .h_stack()
//             .arced();

//         Self {
//             validated_email,
//             components,
//         }
//     }

//     fn validated_email(&self) -> impl Signal<Item = Option<String>> {
//         self.validated_email.signal_cloned()
//     }
// }

// impl Component for EmailForm {
//     fn view(&self) -> floem::AnyView {
//         self.components.view()
//     }
// }

// struct User {
//     name: String,
//     emails: Vec<String>,
// }

// struct UserForm {
//     submits: Events<User>,
//     components: ArcComponent,
// }

// impl UserForm {
//     fn new() -> Self {
//         let name_label = label(signal::always_pending("Name "));
//         let name_input = text_input("", stream::pending::<String>());
//         let name = (name_label, name_input).h_stack();

//         let number_of_emails = Counter::new(2);
//         let email_forms = number_of_emails.value().map(|n| {
//             (1..=n)
//                 .map(|n| EmailForm::new(format!("Email #{} ", n)))
//                 .collect::<Vec<_>>()
//         });
//         let emails = dynamic(email_forms.map(|forms| forms.v_stack()));

//         let submit_button = button("Submit");

//         let components = (name, number_of_emails, emails, submit_button)
//             .v_stack()
//             .arced();

//         let submits = email_forms.sample_stream_cloned(submit_button.clicks()).map(|(forms, ())| {
//             forms.into_iter().map(|f|f.validated_email())
//         })
//     }
// }

// #[derive(Clone)]
// struct Counter {
//     counter: Value<Reactive, i32>,

//     components: ArcComponent,
// }

// impl Counter {
//     fn new(init: i32) -> Self {
//         let increment_button = button("+");
//         let decrement_button = button("-");

//         let changes = events::<i32>();
//         let counter_text_input =
//             text_input(init.to_string(), changes.stream().map(|i| i.to_string()));
//         let counter = counter_text_input
//             .text()
//             .map(|s| s.parse::<i32>().unwrap_or(0))
//             .materialize();
//         let increments = increment_button.clicks().map(|_| 1);
//         let decrements = decrement_button.clicks().map(|_| -1);
//         let deltas = increments.merge(decrements);

//         changes.set(
//             counter
//                 .signal()
//                 .sample_stream_cloned(deltas)
//                 .map(|(counter, delta)| counter + delta),
//         );

//         let buttons = (increment_button, decrement_button).h_stack();
//         let components = (counter_text_input, buttons).v_stack().arced();

//         Self {
//             counter,
//             components,
//         }
//     }

//     pub fn value(&self) -> impl Signal<Item = i32> {
//         self.counter.signal()
//     }
// }

// impl Component for Counter {
//     fn view(&self) -> floem::AnyView {
//         self.components.view()
//     }
// }

#[tokio::main]
async fn main() {}
