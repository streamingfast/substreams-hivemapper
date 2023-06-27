```golang
// database tables
Fleet {
   addr
}

Driver {
   addr
}

FleetDriver {
   fleet_addr
   addr
}

SplitPayments {
   trx_id,
   fleet_payment_id
   driver_payment_id
}

Payments {
   trx_id,
   date,
   to,
   amount,
}

Transfer {
   trx_id,
   date, 
   from, 
   to, 
   amount,
}

// substreams outputs
Payment {
   trx_id,
   date,
   to,
   amount,
}

SplitPayment {
   trx_id,
   date,
   fleet,
   fleet_amount,
   driver,
   driver amount,
}

Transfer {
   trx_id,
   date, 
   from_owner,
   from, 
   to_owner, 
   to, 
   amount,
}
```