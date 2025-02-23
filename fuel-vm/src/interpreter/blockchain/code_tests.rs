use super::*;
use crate::{
    interpreter::{
        memory::Memory,
        PanicContext,
    },
    storage::MemoryStorage,
};
use fuel_tx::Contract;

#[test]
fn test_load_contract() -> Result<(), RuntimeError> {
    let mut storage = MemoryStorage::new(Default::default(), Default::default());
    let mut memory: Memory<MEM_SIZE> = vec![1u8; MEM_SIZE].try_into().unwrap();
    let mut pc = 4;
    let hp = 2000;
    let mut ssp = 1000;
    let mut sp = 1000;
    let fp = 0;

    let contract_id = ContractId::from([4u8; 32]);

    let contract_id_mem_address: Word = 32;
    let offset = 20;
    let num_bytes = 40;

    memory[contract_id_mem_address as usize
        ..contract_id_mem_address as usize + ContractId::LEN]
        .copy_from_slice(contract_id.as_ref());
    storage
        .storage_contract_insert(&contract_id, &Contract::from(vec![5u8; 400]))
        .unwrap();

    let mut panic_context = PanicContext::None;
    let input_contracts = vec![contract_id];
    let input = LoadContractCodeCtx {
        contract_max_size: 100,
        storage: &storage,
        memory: &mut memory,
        input_contracts: InputContracts::new(input_contracts.iter(), &mut panic_context),
        ssp: RegMut::new(&mut ssp),
        sp: RegMut::new(&mut sp),
        fp: Reg::new(&fp),
        hp: Reg::new(&hp),
        pc: RegMut::new(&mut pc),
    };
    input.load_contract_code(contract_id_mem_address, offset, num_bytes)?;
    assert_eq!(pc, 8);

    Ok(())
}

#[test]
fn test_code_copy() -> Result<(), RuntimeError> {
    let mut storage = MemoryStorage::new(Default::default(), Default::default());
    let mut memory: Memory<MEM_SIZE> = vec![1u8; MEM_SIZE].try_into().unwrap();
    let mut pc = 4;

    let contract_id = ContractId::from([4u8; 32]);

    let dest_mem_address = 2001;
    let contract_id_mem_address: Word = 32;
    let offset = 20;
    let num_bytes = 40;

    memory[contract_id_mem_address as usize
        ..contract_id_mem_address as usize + ContractId::LEN]
        .copy_from_slice(contract_id.as_ref());
    storage
        .storage_contract_insert(&contract_id, &Contract::from(vec![5u8; 400]))
        .unwrap();

    let input_contracts = vec![contract_id];
    let mut panic_context = PanicContext::None;
    let input = CodeCopyCtx {
        storage: &storage,
        memory: &mut memory,
        input_contracts: InputContracts::new(input_contracts.iter(), &mut panic_context),
        pc: RegMut::new(&mut pc),
        owner: OwnershipRegisters {
            sp: 1000,
            ssp: 1000,
            hp: 2000,
            prev_hp: VM_MAX_RAM - 1,
            context: Context::Call {
                block_height: Default::default(),
            },
        },
    };
    input.code_copy(dest_mem_address, contract_id_mem_address, offset, num_bytes)?;
    assert_eq!(pc, 8);

    Ok(())
}
