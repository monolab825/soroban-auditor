use tlsh::{BucketKind, Tlsh, ChecksumKind, TlshBuilder, Version};
use regex::Regex;

#[test]
fn test_tlsh() {
    let vec1 = "var_a=global_a-16;var_b=global_a-16;store_i64(var_a+8,1);var_c=vec.vec_new_from_linear_memory(extend_u_i64(var_a+8)<<32|U32(0),U32(1));var_c";

    let vec2= "var_a=0;var_b=global_a-32;var_c=global_a-32;store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=16;var_a+=8{store_i64(var_b+16+var_a,Void);}for var_a=0;var_a==16;var_a+=8{var_d=load_i64(var_a+var_b);store_i64(var_b+16+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+16)<<32|U32(0),U32(2))0";

    let vec3= "var_a=0;var_b=global_a-48;var_c=global_a-48;store_i64(var_b+16,Symbol(World));store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=24;var_a+=8{store_i64(var_b+24+var_a,Void);}for var_a=0;var_a==24;var_a+=8{var_d=load_i64(var_a+var_b);store_i64(var_b+24+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+24)<<32|U32(0),U32(3))0";

    let vec4= "var_a=0;var_b=global_a-64;var_c=global_a-64;store_i64(var_b+24,Symbol(Dong));store_i64(var_b+16,Symbol(World));store_i64(var_b+8,Symbol(Test));store_i64(var_b,Symbol(Hello));for;var_a!=32;var_b=var_a+8{store_i64(var_b+32+var_a,Void);}for var_a=0;var_a==32;var_a+=8{var_d=load_i64(var_b+var_a);store_i64(var_b+32+var_a,var_d);}vec.vec_new_from_linear_memory(extend_u_i64(var_b+32)<<32|U32(0),U32(4))0";

    let total_different = "let var_e=if((arg_c^arg_b>>s63)!=False)|(arg_b--36028797018963968>u72057594037927935){int.obj_from_i128_pieces(arg_c,arg_b)}else{arg_b<<Timepoint(0)|I128(0)};store_i64(arg_a+8,var_e);store_i64(arg_a,False);";

    let (tlsh1, tlsh2, tlsh3, tlsh4, total_different) = build(vec1, vec2, vec3, vec4, total_different);

    //DIFFERENCES
    println!("{:?}", tlsh1.diff(&tlsh2, false)); //149
    println!("{:?}", tlsh1.diff(&tlsh3, false)); //171
    println!("{:?}", tlsh1.diff(&tlsh4, false)); //118
    println!("{:?}", tlsh1.diff(&total_different, false)); //335
    
    println!("{:?}", tlsh2.diff(&tlsh3, false)); //172
    println!("{:?}", tlsh2.diff(&tlsh4, false)); //83 
    println!("{:?}", tlsh3.diff(&tlsh4, false)); //112 
    println!("{:?}", tlsh3.diff(&tlsh4, false)); //11 
}



fn build(vec1: &str, vec2: &str, vec3: &str, vec4: &str, vec5: &str) -> (Tlsh, Tlsh, Tlsh, Tlsh, Tlsh) {
    let mut builder = TlshBuilder::new(BucketKind::Bucket128, ChecksumKind::OneByte, Version::Version4);
    builder.update(vec1.as_bytes());
    let tlsh1 = builder.build().unwrap();

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
    builder.update(vec5.as_bytes());
    let tlsh5 = builder.build().unwrap();

    (tlsh1, tlsh2, tlsh3, tlsh4, tlsh5)
}
