use tlsh::{BucketKind, ChecksumKind, TlshBuilder, Version};

#[test]
fn test_tlsh() {
    let decimals_func = r#"var_a = global_a - 32; var_b = global_a - 32; var_c = func_23(Symbol(METADATA), Void); if var_c { var_d = ledger.get_contract_data(Symbol(METADATA), Void); func_53(var_a, var_d); var_e = load_i64(var_a); if var_e == 0 { var_f = load_32u_i64(var_a + 24); return var_f << 32 | U32(0); } } unreachable!();"#;

    let name_func = r#"var_a = global_a - 32; var_b = global_a - 32; var_c = func_23(Symbol(METADATA), Void); if var_c { var_d = ledger.get_contract_data(Symbol(METADATA), Void); func_53(var_a, var_d); var_e = load_i64(var_a); if var_e == 0 { var_f = load_i64(var_a + 8); return var_f; } } unreachable!();"#;

    let symbol_func = r#"var_a = global_a - 32; var_b = global_a - 32; var_c = func_23(Symbol(METADATA), Void); if var_c { var_d = ledger.get_contract_data(Symbol(METADATA), Void); func_53(var_a, var_d); var_e = load_i64(var_a); if var_e == 0 { var_f = load_i64(var_a + 16); return var_f; } } unreachable!();"#;

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(decimals_func.as_bytes());
    let tlsh1 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(name_func.as_bytes());
    let tlsh2 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(symbol_func.as_bytes());
    let tlsh3 = builder.build().unwrap();

    // Calculate diff between s1 & s2, including length difference.
    let tlsh_res = tlsh1.diff(&tlsh3, true);
    println!("{}", tlsh_res);
    // Calculate diff between s1 & s2, excluding length difference.
    let tlsh_res2 = tlsh1.diff(&tlsh2, false);
    println!("{}", tlsh_res2);

    // Calculate diff between s1 & s2, excluding length difference.
    let tlsh_res3 = tlsh3.diff(&tlsh2, true);
    println!("{}", tlsh_res3);
}
