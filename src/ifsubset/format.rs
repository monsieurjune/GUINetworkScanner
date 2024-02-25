use std::net::Ipv4Addr;
use serde_json::to_string;
use serde::Serialize;

#[derive(Serialize)]
struct JsonSubset
{
    name: String,
    addr_set: Vec<Ipv4Addr>
}

#[derive(Serialize)]
struct JsonFormat
{
    subset: Vec<JsonSubset>
}

fn manage_subset(host_set: &Vec<Vec<Ipv4Addr>>, inter_name: &String) -> Vec<JsonSubset>
{
    let mut output: Vec<JsonSubset> = Vec::new();

    for subset in host_set
    {
        output.push(
            JsonSubset {
                addr_set: subset.clone(),
                name: inter_name.clone()
            }
        )
    }
    output
}

pub fn host_set_to_json(host_set: Vec<Vec<Ipv4Addr>>, inter_name: &String) -> String
{
    let pre_format: JsonFormat = JsonFormat {
        subset: manage_subset(&host_set, inter_name)
    };
    to_string(&pre_format).unwrap()
}