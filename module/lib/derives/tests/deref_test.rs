#![ allow( incomplete_include ) ]

use diagnostics_tools::prelude::*;
use derives::*;

#[ derive( Debug, Clone, Copy, PartialEq, Deref ) ]
pub struct IsTransparent( bool );

include!( "./only_test/deref.rs" );
