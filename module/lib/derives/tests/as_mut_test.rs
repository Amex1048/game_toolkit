#![ allow( incomplete_include ) ]

use diagnostics_tools::prelude::*;
use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, AsMut ) ]
pub struct IsTransparent( bool );

include!( "./only_test/as_mut.rs" );
