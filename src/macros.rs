/// A macro to extract a particular event from a block and map it to a particular protobuf type.
#[macro_export]
macro_rules! map_event_to_proto {
    ($function_name: ident, $event:ty, $singular_proto:ty, $plural_proto_type:ident, $plural_proto_ident:ident, $event_to_proto_closure:expr) => {
        #[substreams::handlers::map]

        pub fn $function_name(
            block: eth::v2::Block,
        ) -> Result<$plural_proto_type, substreams::errors::Error> {
            let $plural_proto_ident: Vec<$singular_proto> = block
                .logs()
                .filter_map(|log| {
                    if crate::format_hex(log.address()) == crate::TELLER_V2.to_lowercase() {
                        let transaction = log.receipt.transaction;

                        if let Some(event) = <$event>::match_and_decode(log) {
                            Some((event, transaction))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .map($event_to_proto_closure)
                .collect();

            Ok($plural_proto_type {
                $plural_proto_ident,
            })
        }
    };
}

// #[macro_export]
// macro_rules! map_event_to_proto {
//     ($function_name: ident, $event:ty, $singular_proto:ty, $plural_proto_type:ident, $plural_proto_ident:ident, $event_to_proto_closure:expr) => {
//         #[substreams::handlers::map]

//         fn $function_name(
//             block: eth::v2::Block,
//         ) -> Result<$plural_proto_type, substreams::errors::Error> {
//             let $plural_proto_ident: Vec<$singular_proto> = block
//                 .logs()
//                 .filter_map(|log| {
//                     if format_hex(log.address()) == TELLER_V2.to_lowercase() {
//                         let transaction = log.receipt.transaction;

//                         if let Some(event) = <$event>::match_and_decode(log) {
//                             Some((event, transaction))
//                         } else {
//                             None
//                         }
//                     } else {
//                         None
//                     }
//                 })
//                 .map($event_to_proto_closure)
//                 .collect();

//             Ok($plural_proto_type {
//                 $plural_proto_ident,
//             })
//         }
//     };
// }

/// A macro to populate columns in a table, and make the remaining required columns their default value
#[macro_export]
macro_rules! populate_columns {
    ($table_enum: ident, $(($enum_variant: ident, $value: expr)),*) => {
        println!("Hi with no rest")
    };

    ($table_enum: ident, $(($enum_variant: ident, $value: expr)),* rest) => {
        println!("Hi with rest")
    };
}
