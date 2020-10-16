use crate::core::system::Read;
use crate::core::system::System;
use crate::core::system::Write;
use crate::core::Store;
use crate::event::input::InputEvent;
use crate::event::stream::EventStream;
use crate::event::stream::SubscriptionToken;
use crate::event::Bindings;
use crate::event::InputHandler;
use crate::event::InputType;

#[derive(Debug)]
pub struct InputSystem<T: InputType> {
    input_stream_token: Option<SubscriptionToken>,
    bindings: Bindings<T>,
}

impl<T: InputType> InputSystem<T> {
    pub fn new(bindings: Bindings<T>) -> Self {
        InputSystem {
            input_stream_token: None,
            bindings,
        }
    }
}

impl<'a, T: InputType> System<'a> for InputSystem<T> {
    type Data = (
        Read<'a, EventStream<InputEvent>>,
        Write<'a, InputHandler<T>>,
    );

    fn init(&mut self, store: &mut Store) {
        if let Some(mut input_stream) = store.get_resource_mut::<EventStream<InputEvent>>() {
            self.input_stream_token = Some(input_stream.subscribe());
        }

        store.insert_resource(InputHandler::<T>::default());
    }

    fn run(&mut self, (event_stream, mut input_handler): Self::Data) {
        if let Some(token) = self.input_stream_token {
            for e in event_stream.read(&token) {
                for action_bindings in self.bindings.action_mappings.iter() {
                    if action_bindings
                        .1
                        .iter()
                        .find(|action| action.source == e.input)
                        .is_some()
                    {
                        input_handler
                            .actions_value
                            .insert(action_bindings.0.clone(), e.value > 0.0);
                    }
                }

                for axes_bindings in self.bindings.axis_mappings.iter() {
                    if let Some(axis) = axes_bindings
                        .1
                        .iter()
                        .find(|action| action.source == e.input)
                    {
                        input_handler
                            .axes_value
                            .insert(axes_bindings.0.clone(), e.value * axis.scale);
                    }
                }
            }
        }
    }

    fn dispose(&mut self, store: &mut Store) {
        if let Some(token) = self.input_stream_token {
            if let Some(mut input_stream) = store.get_resource_mut::<EventStream<InputEvent>>() {
                input_stream.unsubscribe(token);
                self.input_stream_token = None;
            }
        }
    }
}
