#![ allow( incomplete_include ) ]

use diagnostics_tools::prelude::*;
use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, AsRef ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_ref.rs" );
