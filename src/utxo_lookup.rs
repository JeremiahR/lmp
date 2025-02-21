use lightning::routing::utxo::UtxoLookup;

pub struct UtxoLookupImpl;

impl UtxoLookup for UtxoLookupImpl {
    fn get_utxo(&self, txid: &Txid, vout: u32) -> Result<Option<OutPoint>, Error> {
        // Implementation goes here
        unimplemented!()
    }
}
