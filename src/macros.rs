/// A macro to extract a particular event from a block and map it to a particular protobuf type.
#[macro_export]
macro_rules! map_event_to_proto {
    ($function_name: ident, $event:ty, $singular_proto:ty, $plural_proto_type:ident, $plural_proto_ident:ident, $event_to_proto_closure:expr) => {
        #[substreams::handlers::map]
        fn $function_name(
            block: eth::v2::Block,
        ) -> Result<$plural_proto_type, substreams::errors::Error> {
            let $plural_proto_ident: Vec<$singular_proto> = block
                .logs()
                .filter_map(|log| {
                    if format_hex(log.address()) == TELLER_V2.to_lowercase() {
                        if let Some(event) = <$event>::match_and_decode(log) {
                            Some(event)
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
