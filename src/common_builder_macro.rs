#[macro_export]
macro_rules! impl_common_builder {
    ($builder:ty) => {
        impl $builder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn id(mut self, id: String) -> Self {
                self.id = Some(id);
                self
            }
            pub fn to(mut self, value: String) -> Self {
                if let Some(ref mut self_to) = self.to {
                    self_to.push(value);
                } else {
                    self.to = Some(vec![value]);
                }
                self
            }

            pub fn from(mut self, from: String) -> Self {
                self.from = Some(from);
                self
            }

            pub fn created_time(mut self, created_time: Option<u64>) -> Self {
                if let Some(created_time) = created_time {
                    self.created_time = Some(created_time);
                } else {
                    self.created_time = Some(chrono::Utc::now().timestamp() as u64);
                }
                self
            }

            pub fn expires_time(mut self, expires_time: u64) -> Self {
                self.expires_time = Some(expires_time);
                self
            }

            fn add_common_to_builder(
                self,
                mut didcomm_msg_builder: MessageBuilder,
            ) -> Result<MessageBuilder, MessageBuilderError> {
                if let Some(to) = self.to {
                    for value in to {
                        didcomm_msg_builder = didcomm_msg_builder.to(value);
                    }
                }

                if let Some(from) = self.from {
                    didcomm_msg_builder = didcomm_msg_builder.from(from);
                }

                if let Some(created_time) = self.created_time {
                    didcomm_msg_builder = didcomm_msg_builder.created_time(created_time);
                }

                if let Some(expires_time) = self.expires_time {
                    didcomm_msg_builder = didcomm_msg_builder.expires_time(expires_time);
                }

                Ok(didcomm_msg_builder)
            }
        }
    };
}
