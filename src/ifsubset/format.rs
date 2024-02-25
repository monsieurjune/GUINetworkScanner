use std::net::Ipv4Addr;
use serde_json::to_string;
use serde::Serialize;

#[derive(Serialize)]
struct JsonSubset
{
    length: usize,
    addr_set: Vec<Ipv4Addr>
}

#[derive(Serialize)]
struct JsonFormat
{
    length: usize,
    name: String,
    subset: Vec<JsonSubset>
}

fn manage_subset(host_set: &Vec<Vec<Ipv4Addr>>) -> Vec<JsonSubset>
{
    let mut output: Vec<JsonSubset> = Vec::new();

    for subset in host_set
    {
        output.push(
            JsonSubset {
                length: subset.len(),
                addr_set: subset.clone()
            }
        )
    }
    output
}

pub fn host_set_to_json(host_set: Vec<Vec<Ipv4Addr>>, inter_name: &String) -> String
{
    let pre_format: JsonFormat = JsonFormat {
        length: host_set.len(),
        name: inter_name.clone(),
        subset: manage_subset(&host_set)
    };
    to_string(&pre_format).unwrap()
}