use tlsh::{BucketKind, Tlsh, ChecksumKind, TlshBuilder, Version};
use lcs::LcsTable;

#[test]
fn test_tlsh() {
    let vec_pattern = "var_a=global_a-;var_=global_a-16;store_i64(var_a+8,);var_=vec.vec_new_from_linear_memory(extend_u_i64(var_+)<<32|U32(0),U32()";


    let vec1 = "var_a=global_a-16;var_b=global_a-16;store_i64(var_a+8,1);var_c=vec.vec_new_from_linear_memory(extend_u_i64(var_a+8)<<32|U32(0),U32(1));var_c";
    let vec2= "var_a=0;var_b=global_a-32;var_c=global_a-32;store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=16;var_a+=8{store_i64(var_b+16+var_a,Void);}for var_a=0;var_a==16;var_a+=8{var_d=load_i64(var_a+var_b);store_i64(var_b+16+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+16)<<32|U32(0),U32(2))0";
    let vec3= "var_a=0;var_b=global_a-48;var_c=global_a-48;store_i64(var_b+16,Symbol(World));store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=24;var_a+=8{store_i64(var_b+24+var_a,Void);}for var_a=0;var_a==24;var_a+=8{var_d=load_i64(var_a+var_b);store_i64(var_b+24+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+24)<<32|U32(0),U32(3))0";
    let vec4= "var_a=0;var_b=global_a-64;var_c=global_a-64;store_i64(var_b+24,Symbol(Dong));store_i64(var_b+16,Symbol(World));store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=32;var_b=var_a+8{store_i64(var_b+32+var_a,Void);}for var_a=0;var_a==32;var_a+=8{var_d=load_i64(var_b+var_a);store_i64(var_b+32+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+32)<<32|U32(0),U32(4))0";
    let total_different = "let var_e=if((arg_c^arg_b>>s63)!=False)|(arg_b--36028797018963968>u72057594037927935){int.obj_from_i128_pieces(arg_c,arg_b)}else{arg_b<<Timepoint(0)|I128(0)};store_i64(arg_a+8,var_e);store_i64(arg_a,False);";


    let (tlsh1, tlsh2, tlsh3, tlsh4) = build(vec_pattern, vec2, vec3, vec4, total_different);

    //DIFFERENCES
    // println!("{:?}", tlsh1.hash());
    // println!("{:?}", tlsh2.hash());
    // println!("{:?}", tlsh3.hash());
    // println!("TLSH5: {:?}", tlsh5.hash());

    //LCS
}

fn check_lcs_patterns(function_body: &str, pattern: &str) -> String {
    let a: Vec<_> = function_body.chars().collect();
    let b: Vec<_> = pattern.chars().collect();
    let table = LcsTable::new(&a, &b);
    let lcs = table.longest_common_subsequence();

    let formated = lcs
    .iter()
    .map(|&(c1, _)| c1)
    .collect::<String>();
    format!("{:?}", formated)
}



fn build(vec_pattern: &str, vec2: &str, vec3: &str, vec4: &str, vec5: &str) -> (Tlsh, Tlsh, Tlsh, Tlsh) {

    let pattern1 = check_lcs_patterns(vec2, vec3);
    let pattern2 = check_lcs_patterns(vec4, vec_pattern);
    let pattern3 = check_lcs_patterns(vec5, vec4);

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(vec2.as_bytes());
    let tlsh2 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(vec3.as_bytes());
    let tlsh3 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(vec4.as_bytes());
    let tlsh4 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(pattern1.as_bytes());
    let tlsh5 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(pattern2.as_bytes());
    let tlsh6 = builder.build().unwrap();

    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(vec_pattern.as_bytes());
    let tlsh7 = builder.build().unwrap();

    println!("Pattern code for Vec[] {:?}", vec_pattern);
    println!("Generate for pattern token hash: {:?}", tlsh7.hash());

    println!("Function Body {:?}", vec4);
    println!("Found Pattern Code {:?}", pattern2);
    println!("Generate Token Hash: {:?}", tlsh6.hash());

    println!("Token Diff Between pattern and found pattern: {:?}", tlsh6.diff(&tlsh7, false));

    (tlsh2, tlsh3, tlsh4, tlsh5)
}
