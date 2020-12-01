use super::Metric;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

mod protobuf;
mod buffer;

pub fn parse(msg: Vec<u8>) -> Vec<Metric> {
    println!("{:?}", msg);
    // vec![Metric::new(
    //     "test".to_string(),
    //     HashMap::new(),
    //     0.0,
    //     SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .expect("Time went backwards")
    //         .as_secs(),
    // )]

    let mut metric: Vec<Metric> = Vec::new();
    let ts: Vec<u8> = Vec::new();
    let sample: Vec<u8> = Vec::new();
    let metric_buffer = MetricBuffer::new();
    let metric: Vec<String> = Vec::new();
    let mut sameples_offset = 0;
    let value: f64 = 0.0;
    let timestamp: i64 = 0;


    'time_series: while msg.len() > 0 {
        if b[0] != 0x0a { // repeated prometheus.TimeSeries timeseries = 1;
            if b, err = pb.Skip(b); err != nil {
                break 'time_series;
            }
        }

        if ts, b, err = pb.Bytes(b[1:]); err != nil {
            break 'time_series;
        }

        if metric, samplesOffset, err = metricBuffer.timeSeries(ts); err != nil {
            break 'time_series;
        }

        ts = ts[samplesOffset:];
        'samples: for len(ts) > 0 {
            if ts[0] != 0x12 { // repeated Sample samples = 2;
                if ts, err = pb.Skip(ts); err != nil {
                    break 'time_series;
                }

                    if sample, ts, err = pb.Bytes(ts[1:]); err != nil {
                            break TimeSeriesLoop
                    }

                    timestamp = 0
                    value = 0

                    for len(sample) > 0 {
                            switch sample[0] {
                            case 0x09: // double value    = 1;
                                    if value, sample, err = pb.Double(sample[1:]); err != nil {
                                            break TimeSeriesLoop
                                    }
                            case 0x10: // int64 timestamp = 2;
                                    if timestamp, sample, err = pb.Int64(sample[1:]); err != nil {
                                            break TimeSeriesLoop
                                    }
                            default:
                                    if sample, err = pb.Skip(sample); err != nil {
                                            break TimeSeriesLoop
                                    }
                            }
                    }

                    if math.IsNaN(value) {
                            continue SamplesLoop
                    }

                    if rcv.isDropString("", writer.Now(), uint32(timestamp/1000), value) {
                            continue
                    }

                    metrics.push(Metric::new(metric, tags, value, timestamp/1000));
            }
    };
    metrics
}
