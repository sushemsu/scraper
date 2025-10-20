use crate::pulse::types::Chart;
use postgres::Client;
const DAY: i64 = 86400;
const SECOND: i64 = 1000;
const DAYS_TO_RETAIN: i64 = 30;


pub fn insert_chart(client: &mut Client, host: &str, chart: &Chart) {
    match client.query("SELECT host FROM charts WHERE host = $1", &[&host]) {
        Ok(result) => result.len(),
        Err(..) => 0,
    };

    let retention = &chart.cpu[0].timestamp - (DAYS_TO_RETAIN * DAY * SECOND);
    // trying to rotate logs
    //match client.batch_execute("WITH RankedRows AS (SELECT host, ROW_NUMBER() OVER (ORDER BY tstamp DESC) as rn FROM charts) DELETE FROM charts WHERE host IN (SELECT host FROM RankedRows WHERE rn > 50);"){
    match client.execute("delete from charts where tstamp < $1", &[&retention]){
        Ok(..) => {} Err(error) => {println!("Err: {}", error);}
    };

    // found a row do update statement
    /*if rows > 0 {
        match client.execute("UPDATE charts SET cpu = $2, disk = $3, diskread = $4, diskwrite = $5, memory = $6, netin = $7, netout = $8 WHERE host = $1",  &[&host, &chart.cpu[0].value, &chart.disk[0].value, &chart.diskread[0].value, &chart.diskwrite[0].value, &chart.memory[0].value, &chart.netin[0].value, &chart.netout[0].value]){
                Ok(value) => {
                    value
                }
                Err(error) => {
                    println!("Err updating table: {}", error);
                    0
                }
        };
    } else {*/
        match client.execute("INSERT INTO charts (host, cpu, disk, diskread, diskwrite, memory, netin, netout, tstamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)", &[&host, &chart.cpu[0].value, &chart.disk[0].value, &chart.diskread[0].value, &chart.diskwrite[0].value, &chart.memory[0].value, &chart.netin[0].value, &chart.netout[0].value, &chart.cpu[0].timestamp]) {
                Ok(value) => {
                    value
                }
                Err(error) => {
                    println!("Err inserting into table: {}", error);
                    0
                }
        };
    //}
}
