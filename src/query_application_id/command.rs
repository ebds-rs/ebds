use crate::{
    impl_aux_ops, impl_message_ops, len::QUERY_APPLICATION_ID_COMMAND, AuxCommand, AuxCommandOps,
    MessageOps, MessageType,
};

/// Query Application ID - Command (Subtype 0x0E)
///
/// This command is used to return the software part number of the actual application component of the
/// device firmware.
///
/// This is only applicable when the device has been loaded with a combine file (a file that
/// contains both the application and the variant) because the (Subtype 0x07) Query Acceptor Application
/// Part Number command will return the part number of the combine file, not the underlying component’s
/// part number.
///
/// The device capabilities map (section 7.4.14) has an entry as to whether or not the device
/// supports this command.
///
/// The Query Application ID Command is formatted as follows:
///
/// | Name  | STX  | LEN  | CTRL | Data A | Data B | Command | ETX  | CHK |
/// |:------|:----:|:----:|:----:|:------:|:------:|:-------:|:----:|:---:|
/// | Byte  | 0    | 1    | 2    | 3      | 4      | 5       | 6    | 7   |
/// | Value | 0x02 | 0x08 | 0x6n | 0x00   | 0x00   | 0x0E    | 0x03 | zz  |
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QueryApplicationIdCommand {
    buf: [u8; QUERY_APPLICATION_ID_COMMAND],
}

impl QueryApplicationIdCommand {
    pub fn new() -> Self {
        let mut message = Self {
            buf: [0u8; QUERY_APPLICATION_ID_COMMAND],
        };

        message.init();
        message.set_message_type(MessageType::AuxCommand);
        message.set_aux_command(AuxCommand::QueryApplicationId);

        message
    }
}

impl_message_ops!(QueryApplicationIdCommand);
impl_aux_ops!(QueryApplicationIdCommand);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    #[rustfmt::skip]
    fn test_query_application_id_command_from_buf() -> Result<()> {
        let msg_bytes = [
            // STX | LEN | Message Type
            0x02, 0x08, 0x60,
            // Data
            0x00, 0x00,
            // Command
            0x0e,
            // ETX | Checksum
            0x03, 0x66,
        ];

        let mut msg = QueryApplicationIdCommand::new();
        msg.from_buf(msg_bytes.as_ref())?;

        assert_eq!(msg.message_type(), MessageType::AuxCommand);
        assert_eq!(msg.aux_command(), AuxCommand::QueryApplicationId);

        Ok(())
    }
}
