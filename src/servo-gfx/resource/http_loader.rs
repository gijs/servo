export factory;

use comm::Chan;
use task::spawn;
use resource_task::{ProgressMsg, Payload, Done};
use std::net::url::Url;
use http_client::{uv_http_request};

pub fn factory(url: Url, progress_chan: Chan<ProgressMsg>) {
    assert url.scheme == ~"http";

    do spawn |move url| {
        debug!("http_loader: requesting via http: %?", copy url);
        let request = uv_http_request(copy url);
        let errored = @mut false;
        do request.begin |event, copy url| {
            let url = copy url;
            match event {
                http_client::Status(*) => { }
                http_client::Payload(data) => {
                    debug!("http_loader: got data from %?", url);
                    let mut junk = None;
                    *data <-> junk;
                    progress_chan.send(Payload(option::unwrap(move junk)));
                }
                http_client::Error(*) => {
                    debug!("http_loader: error loading %?", url);
                    *errored = true;
                    progress_chan.send(Done(Err(())));
                }
            }
        }

        if !*errored {
            progress_chan.send(Done(Ok(())));
        }
    }
}
