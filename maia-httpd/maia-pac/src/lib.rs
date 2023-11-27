# ! [doc = "Peripheral access API for MAIA SDR microcontrollers (generated using svd2rust v0.31.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.31.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
# ! [allow (non_camel_case_types)]
# ! [allow (non_snake_case)]
# ! [no_std]
use core :: ops :: Deref ; use core :: marker :: PhantomData ; # [allow (unused_imports)]
use generic :: * ; # [doc = r"Common register and bit access and modify traits"]
pub mod generic { use core :: marker ; # [doc = " Raw register type (`u8`, `u16`, `u32`, ...)"]
pub trait RawReg : Copy + Default + From < bool > + core :: ops :: BitOr < Output = Self > + core :: ops :: BitAnd < Output = Self > + core :: ops :: BitOrAssign + core :: ops :: BitAndAssign + core :: ops :: Not < Output = Self > + core :: ops :: Shl < u8 , Output = Self > { # [doc = " Mask for bits of width `WI`"]
fn mask < const WI : u8 > () -> Self ; # [doc = " Mask for bits of width 1"]
fn one () -> Self ; } macro_rules ! raw_reg { ($ U : ty , $ size : literal , $ mask : ident) => { impl RawReg for $ U { # [inline (always)]
fn mask < const WI : u8 > () -> Self { $ mask ::< WI > () } # [inline (always)]
fn one () -> Self { 1 } } const fn $ mask < const WI : u8 > () -> $ U { <$ U >:: MAX >> ($ size - WI) } impl FieldSpec for $ U { type Ux = $ U ; } } ; } raw_reg ! (u8 , 8 , mask_u8) ; raw_reg ! (u16 , 16 , mask_u16) ; raw_reg ! (u32 , 32 , mask_u32) ; raw_reg ! (u64 , 64 , mask_u64) ; # [doc = " Raw register type"]
pub trait RegisterSpec { # [doc = " Raw register type (`u8`, `u16`, `u32`, ...)."]
type Ux : RawReg ; } # [doc = " Raw field type"]
pub trait FieldSpec : Sized { # [doc = " Raw field type (`u8`, `u16`, `u32`, ...)."]
type Ux : Copy + PartialEq + From < Self > ; } # [doc = " Trait implemented by readable registers to enable the `read` method."]
# [doc = ""]
# [doc = " Registers marked with `Writable` can be also be `modify`'ed."]
pub trait Readable : RegisterSpec { } # [doc = " Trait implemented by writeable registers."]
# [doc = ""]
# [doc = " This enables the  `write`, `write_with_zero` and `reset` methods."]
# [doc = ""]
# [doc = " Registers marked with `Readable` can be also be `modify`'ed."]
pub trait Writable : RegisterSpec { # [doc = " Specifies the register bits that are not changed if you pass `1` and are changed if you pass `0`"]
const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux ; # [doc = " Specifies the register bits that are not changed if you pass `0` and are changed if you pass `1`"]
const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux ; } # [doc = " Reset value of the register."]
# [doc = ""]
# [doc = " This value is the initial value for the `write` method. It can also be directly written to the"]
# [doc = " register by using the `reset` method."]
pub trait Resettable : RegisterSpec { # [doc = " Reset value of the register."]
const RESET_VALUE : Self :: Ux ; # [doc = " Reset value of the register."]
# [inline (always)]
fn reset_value () -> Self :: Ux { Self :: RESET_VALUE } } # [doc = " This structure provides volatile access to registers."]
# [repr (transparent)]
pub struct Reg < REG : RegisterSpec > { register : vcell :: VolatileCell < REG :: Ux > , _marker : marker :: PhantomData < REG > , } unsafe impl < REG : RegisterSpec > Send for Reg < REG > where REG :: Ux : Send { } impl < REG : RegisterSpec > Reg < REG > { # [doc = " Returns the underlying memory address of register."]
# [doc = ""]
# [doc = " ```ignore"]
# [doc = " let reg_ptr = periph.reg.as_ptr();"]
# [doc = " ```"]
# [inline (always)]
pub fn as_ptr (& self) -> * mut REG :: Ux { self . register . as_ptr () } } impl < REG : Readable > Reg < REG > { # [doc = " Reads the contents of a `Readable` register."]
# [doc = ""]
# [doc = " You can read the raw contents of a register by using `bits`:"]
# [doc = " ```ignore"]
# [doc = " let bits = periph.reg.read().bits();"]
# [doc = " ```"]
# [doc = " or get the content of a particular field of a register:"]
# [doc = " ```ignore"]
# [doc = " let reader = periph.reg.read();"]
# [doc = " let bits = reader.field1().bits();"]
# [doc = " let flag = reader.field2().bit_is_set();"]
# [doc = " ```"]
# [inline (always)]
pub fn read (& self) -> R < REG > { R { bits : self . register . get () , _reg : marker :: PhantomData , } } } impl < REG : Resettable + Writable > Reg < REG > { # [doc = " Writes the reset value to `Writable` register."]
# [doc = ""]
# [doc = " Resets the register to its initial state."]
# [inline (always)]
pub fn reset (& self) { self . register . set (REG :: RESET_VALUE) } # [doc = " Writes bits to a `Writable` register."]
# [doc = ""]
# [doc = " You can write raw bits into a register:"]
# [doc = " ```ignore"]
# [doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });"]
# [doc = " ```"]
# [doc = " or write only the fields you need:"]
# [doc = " ```ignore"]
# [doc = " periph.reg.write(|w| w"]
# [doc = "     .field1().bits(newfield1bits)"]
# [doc = "     .field2().set_bit()"]
# [doc = "     .field3().variant(VARIANT)"]
# [doc = " );"]
# [doc = " ```"]
# [doc = " or an alternative way of saying the same:"]
# [doc = " ```ignore"]
# [doc = " periph.reg.write(|w| {"]
# [doc = "     w.field1().bits(newfield1bits);"]
# [doc = "     w.field2().set_bit();"]
# [doc = "     w.field3().variant(VARIANT)"]
# [doc = " });"]
# [doc = " ```"]
# [doc = " In the latter case, other fields will be set to their reset value."]
# [inline (always)]
pub fn write < F > (& self , f : F) where F : FnOnce (& mut W < REG >) -> & mut W < REG > , { self . register . set (f (& mut W { bits : REG :: RESET_VALUE & ! REG :: ONE_TO_MODIFY_FIELDS_BITMAP | REG :: ZERO_TO_MODIFY_FIELDS_BITMAP , _reg : marker :: PhantomData , }) . bits ,) ; } } impl < REG : Writable > Reg < REG > { # [doc = " Writes 0 to a `Writable` register."]
# [doc = ""]
# [doc = " Similar to `write`, but unused bits will contain 0."]
# [doc = ""]
# [doc = " # Safety"]
# [doc = ""]
# [doc = " Unsafe to use with registers which don't allow to write 0."]
# [inline (always)]
pub unsafe fn write_with_zero < F > (& self , f : F) where F : FnOnce (& mut W < REG >) -> & mut W < REG > , { self . register . set (f (& mut W { bits : REG :: Ux :: default () , _reg : marker :: PhantomData , }) . bits ,) ; } } impl < REG : Readable + Writable > Reg < REG > { # [doc = " Modifies the contents of the register by reading and then writing it."]
# [doc = ""]
# [doc = " E.g. to do a read-modify-write sequence to change parts of a register:"]
# [doc = " ```ignore"]
# [doc = " periph.reg.modify(|r, w| unsafe { w.bits("]
# [doc = "    r.bits() | 3"]
# [doc = " ) });"]
# [doc = " ```"]
# [doc = " or"]
# [doc = " ```ignore"]
# [doc = " periph.reg.modify(|_, w| w"]
# [doc = "     .field1().bits(newfield1bits)"]
# [doc = "     .field2().set_bit()"]
# [doc = "     .field3().variant(VARIANT)"]
# [doc = " );"]
# [doc = " ```"]
# [doc = " or an alternative way of saying the same:"]
# [doc = " ```ignore"]
# [doc = " periph.reg.modify(|_, w| {"]
# [doc = "     w.field1().bits(newfield1bits);"]
# [doc = "     w.field2().set_bit();"]
# [doc = "     w.field3().variant(VARIANT)"]
# [doc = " });"]
# [doc = " ```"]
# [doc = " Other fields will have the value they had before the call to `modify`."]
# [inline (always)]
pub fn modify < F > (& self , f : F) where for < 'w > F : FnOnce (& R < REG > , & 'w mut W < REG >) -> & 'w mut W < REG > , { let bits = self . register . get () ; self . register . set (f (& R { bits , _reg : marker :: PhantomData , } , & mut W { bits : bits & ! REG :: ONE_TO_MODIFY_FIELDS_BITMAP | REG :: ZERO_TO_MODIFY_FIELDS_BITMAP , _reg : marker :: PhantomData , } ,) . bits ,) ; } } # [doc (hidden)]
pub mod raw { use super :: { marker , BitM , FieldSpec , RegisterSpec , Unsafe , Writable } ; pub struct R < REG : RegisterSpec > { pub (crate) bits : REG :: Ux , pub (super) _reg : marker :: PhantomData < REG > , } pub struct W < REG : RegisterSpec > { # [doc = "Writable bits"]
pub (crate) bits : REG :: Ux , pub (super) _reg : marker :: PhantomData < REG > , } pub struct FieldReader < FI = u8 > where FI : FieldSpec , { pub (crate) bits : FI :: Ux , _reg : marker :: PhantomData < FI > , } impl < FI : FieldSpec > FieldReader < FI > { # [doc = " Creates a new instance of the reader."]
# [allow (unused)]
# [inline (always)]
pub (crate) const fn new (bits : FI :: Ux) -> Self { Self { bits , _reg : marker :: PhantomData , } } } pub struct BitReader < FI = bool > { pub (crate) bits : bool , _reg : marker :: PhantomData < FI > , } impl < FI > BitReader < FI > { # [doc = " Creates a new instance of the reader."]
# [allow (unused)]
# [inline (always)]
pub (crate) const fn new (bits : bool) -> Self { Self { bits , _reg : marker :: PhantomData , } } } pub struct FieldWriter < 'a , REG , const WI : u8 , FI = u8 , Safety = Unsafe > where REG : Writable + RegisterSpec , FI : FieldSpec , { pub (crate) w : & 'a mut W < REG > , pub (crate) o : u8 , _field : marker :: PhantomData < (FI , Safety) > , } impl < 'a , REG , const WI : u8 , FI , Safety > FieldWriter < 'a , REG , WI , FI , Safety > where REG : Writable + RegisterSpec , FI : FieldSpec , { # [doc = " Creates a new instance of the writer"]
# [allow (unused)]
# [inline (always)]
pub (crate) fn new (w : & 'a mut W < REG > , o : u8) -> Self { Self { w , o , _field : marker :: PhantomData , } } } pub struct BitWriter < 'a , REG , FI = bool , M = BitM > where REG : Writable + RegisterSpec , bool : From < FI > , { pub (crate) w : & 'a mut W < REG > , pub (crate) o : u8 , _field : marker :: PhantomData < (FI , M) > , } impl < 'a , REG , FI , M > BitWriter < 'a , REG , FI , M > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = " Creates a new instance of the writer"]
# [allow (unused)]
# [inline (always)]
pub (crate) fn new (w : & 'a mut W < REG > , o : u8) -> Self { Self { w , o , _field : marker :: PhantomData , } } } } # [doc = " Register reader."]
# [doc = ""]
# [doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`"]
# [doc = " method."]
pub type R < REG > = raw :: R < REG > ; impl < REG : RegisterSpec > R < REG > { # [doc = " Reads raw bits from register."]
# [inline (always)]
pub const fn bits (& self) -> REG :: Ux { self . bits } } impl < REG : RegisterSpec , FI > PartialEq < FI > for R < REG > where REG :: Ux : PartialEq , FI : Copy , REG :: Ux : From < FI > , { # [inline (always)]
fn eq (& self , other : & FI) -> bool { self . bits . eq (& REG :: Ux :: from (* other)) } } # [doc = " Register writer."]
# [doc = ""]
# [doc = " Used as an argument to the closures in the `write` and `modify` methods of the register."]
pub type W < REG > = raw :: W < REG > ; # [doc = " Field reader."]
# [doc = ""]
# [doc = " Result of the `read` methods of fields."]
pub type FieldReader < FI = u8 > = raw :: FieldReader < FI > ; # [doc = " Bit-wise field reader"]
pub type BitReader < FI = bool > = raw :: BitReader < FI > ; impl < FI : FieldSpec > FieldReader < FI > { # [doc = " Reads raw bits from field."]
# [inline (always)]
pub const fn bits (& self) -> FI :: Ux { self . bits } } impl < FI > PartialEq < FI > for FieldReader < FI > where FI : FieldSpec + Copy , { # [inline (always)]
fn eq (& self , other : & FI) -> bool { self . bits . eq (& FI :: Ux :: from (* other)) } } impl < FI > PartialEq < FI > for BitReader < FI > where FI : Copy , bool : From < FI > , { # [inline (always)]
fn eq (& self , other : & FI) -> bool { self . bits . eq (& bool :: from (* other)) } } impl < FI > BitReader < FI > { # [doc = " Value of the field as raw bits."]
# [inline (always)]
pub const fn bit (& self) -> bool { self . bits } # [doc = " Returns `true` if the bit is clear (0)."]
# [inline (always)]
pub const fn bit_is_clear (& self) -> bool { ! self . bit () } # [doc = " Returns `true` if the bit is set (1)."]
# [inline (always)]
pub const fn bit_is_set (& self) -> bool { self . bit () } } # [doc (hidden)]
pub struct Safe ; # [doc (hidden)]
pub struct Unsafe ; # [doc = " Write field Proxy with unsafe `bits`"]
pub type FieldWriter < 'a , REG , const WI : u8 , FI = u8 > = raw :: FieldWriter < 'a , REG , WI , FI , Unsafe > ; # [doc = " Write field Proxy with safe `bits`"]
pub type FieldWriterSafe < 'a , REG , const WI : u8 , FI = u8 > = raw :: FieldWriter < 'a , REG , WI , FI , Safe > ; impl < 'a , REG , const WI : u8 , FI > FieldWriter < 'a , REG , WI , FI > where REG : Writable + RegisterSpec , FI : FieldSpec , REG :: Ux : From < FI :: Ux > , { # [doc = " Field width"]
pub const WIDTH : u8 = WI ; # [doc = " Field width"]
# [inline (always)]
pub const fn width (& self) -> u8 { WI } # [doc = " Field offset"]
# [inline (always)]
pub const fn offset (& self) -> u8 { self . o } # [doc = " Writes raw bits to the field"]
# [doc = ""]
# [doc = " # Safety"]
# [doc = ""]
# [doc = " Passing incorrect value can cause undefined behaviour. See reference manual"]
# [inline (always)]
pub unsafe fn bits (self , value : FI :: Ux) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: mask :: < WI > () << self . o) ; self . w . bits |= (REG :: Ux :: from (value) & REG :: Ux :: mask :: < WI > ()) << self . o ; self . w } # [doc = " Writes `variant` to the field"]
# [inline (always)]
pub fn variant (self , variant : FI) -> & 'a mut W < REG > { unsafe { self . bits (FI :: Ux :: from (variant)) } } } impl < 'a , REG , const WI : u8 , FI > FieldWriterSafe < 'a , REG , WI , FI > where REG : Writable + RegisterSpec , FI : FieldSpec , REG :: Ux : From < FI :: Ux > , { # [doc = " Field width"]
pub const WIDTH : u8 = WI ; # [doc = " Field width"]
# [inline (always)]
pub const fn width (& self) -> u8 { WI } # [doc = " Field offset"]
# [inline (always)]
pub const fn offset (& self) -> u8 { self . o } # [doc = " Writes raw bits to the field"]
# [inline (always)]
pub fn bits (self , value : FI :: Ux) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: mask :: < WI > () << self . o) ; self . w . bits |= (REG :: Ux :: from (value) & REG :: Ux :: mask :: < WI > ()) << self . o ; self . w } # [doc = " Writes `variant` to the field"]
# [inline (always)]
pub fn variant (self , variant : FI) -> & 'a mut W < REG > { self . bits (FI :: Ux :: from (variant)) } } macro_rules ! bit_proxy { ($ writer : ident , $ mwv : ident) => { # [doc (hidden)]
pub struct $ mwv ; # [doc = " Bit-wise write field proxy"]
pub type $ writer <'a , REG , FI = bool > = raw :: BitWriter <'a , REG , FI , $ mwv >; impl <'a , REG , FI > $ writer <'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI >, { # [doc = " Field width"]
pub const WIDTH : u8 = 1 ; # [doc = " Field width"]
# [inline (always)]
pub const fn width (& self) -> u8 { Self :: WIDTH } # [doc = " Field offset"]
# [inline (always)]
pub const fn offset (& self) -> u8 { self . o } # [doc = " Writes bit to the field"]
# [inline (always)]
pub fn bit (self , value : bool) -> &'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: one () << self . o) ; self . w . bits |= (REG :: Ux :: from (value) & REG :: Ux :: one ()) << self . o ; self . w } # [doc = " Writes `variant` to the field"]
# [inline (always)]
pub fn variant (self , variant : FI) -> &'a mut W < REG > { self . bit (bool :: from (variant)) } } } ; } bit_proxy ! (BitWriter , BitM) ; bit_proxy ! (BitWriter1S , Bit1S) ; bit_proxy ! (BitWriter0C , Bit0C) ; bit_proxy ! (BitWriter1C , Bit1C) ; bit_proxy ! (BitWriter0S , Bit0S) ; bit_proxy ! (BitWriter1T , Bit1T) ; bit_proxy ! (BitWriter0T , Bit0T) ; impl < 'a , REG , FI > BitWriter < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = " Sets the field bit"]
# [inline (always)]
pub fn set_bit (self) -> & 'a mut W < REG > { self . w . bits |= REG :: Ux :: one () << self . o ; self . w } # [doc = " Clears the field bit"]
# [inline (always)]
pub fn clear_bit (self) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: one () << self . o) ; self . w } } impl < 'a , REG , FI > BitWriter1S < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = " Sets the field bit"]
# [inline (always)]
pub fn set_bit (self) -> & 'a mut W < REG > { self . w . bits |= REG :: Ux :: one () << self . o ; self . w } } impl < 'a , REG , FI > BitWriter0C < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = " Clears the field bit"]
# [inline (always)]
pub fn clear_bit (self) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: one () << self . o) ; self . w } } impl < 'a , REG , FI > BitWriter1C < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = "Clears the field bit by passing one"]
# [inline (always)]
pub fn clear_bit_by_one (self) -> & 'a mut W < REG > { self . w . bits |= REG :: Ux :: one () << self . o ; self . w } } impl < 'a , REG , FI > BitWriter0S < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = "Sets the field bit by passing zero"]
# [inline (always)]
pub fn set_bit_by_zero (self) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: one () << self . o) ; self . w } } impl < 'a , REG , FI > BitWriter1T < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = "Toggle the field bit by passing one"]
# [inline (always)]
pub fn toggle_bit (self) -> & 'a mut W < REG > { self . w . bits |= REG :: Ux :: one () << self . o ; self . w } } impl < 'a , REG , FI > BitWriter0T < 'a , REG , FI > where REG : Writable + RegisterSpec , bool : From < FI > , { # [doc = "Toggle the field bit by passing zero"]
# [inline (always)]
pub fn toggle_bit (self) -> & 'a mut W < REG > { self . w . bits &= ! (REG :: Ux :: one () << self . o) ; self . w } } } # [doc = "Maia SDR IP core"]
pub struct MAIA_SDR { _marker : PhantomData < * const () > } unsafe impl Send for MAIA_SDR { } impl MAIA_SDR { # [doc = r"Pointer to the register block"]
pub const PTR : * const maia_sdr :: RegisterBlock = 0 as * const _ ; # [doc = r"Return the pointer to the register block"]
# [inline (always)]
pub const fn ptr () -> * const maia_sdr :: RegisterBlock { Self :: PTR } # [doc = r" Steal an instance of this peripheral"]
# [doc = r""]
# [doc = r" # Safety"]
# [doc = r""]
# [doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
# [doc = r" that may race with any existing instances, for example by only"]
# [doc = r" accessing read-only or write-only registers, or by consuming the"]
# [doc = r" original peripheral and using critical sections to coordinate"]
# [doc = r" access between multiple new instances."]
# [doc = r""]
# [doc = r" Additionally, other software such as HALs may rely on only one"]
# [doc = r" peripheral instance existing to ensure memory safety; ensure"]
# [doc = r" no stolen instances are passed to such software."]
pub unsafe fn steal () -> Self { Self { _marker : PhantomData } } } impl Deref for MAIA_SDR { type Target = maia_sdr :: RegisterBlock ; # [inline (always)]
fn deref (& self) -> & Self :: Target { unsafe { & * Self :: PTR } } } impl core :: fmt :: Debug for MAIA_SDR { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("MAIA_SDR") . finish () } } # [doc = "Maia SDR IP core"]
pub mod maia_sdr { # [doc = r"Register block"]
# [repr (C)]
pub struct RegisterBlock { product_id : PRODUCT_ID , version : VERSION , control : CONTROL , interrupts : INTERRUPTS , recorder_control : RECORDER_CONTROL , recorder_next_address : RECORDER_NEXT_ADDRESS , _reserved6 : [u8 ; 0x08]
, spectrometer : SPECTROMETER , } impl RegisterBlock { # [doc = "0x00 - product_id"]
# [inline (always)]
pub const fn product_id (& self) -> & PRODUCT_ID { & self . product_id } # [doc = "0x04 - version"]
# [inline (always)]
pub const fn version (& self) -> & VERSION { & self . version } # [doc = "0x08 - control"]
# [inline (always)]
pub const fn control (& self) -> & CONTROL { & self . control } # [doc = "0x0c - interrupts"]
# [inline (always)]
pub const fn interrupts (& self) -> & INTERRUPTS { & self . interrupts } # [doc = "0x10 - recorder_control"]
# [inline (always)]
pub const fn recorder_control (& self) -> & RECORDER_CONTROL { & self . recorder_control } # [doc = "0x14 - recorder_next_address"]
# [inline (always)]
pub const fn recorder_next_address (& self) -> & RECORDER_NEXT_ADDRESS { & self . recorder_next_address } # [doc = "0x20 - spectrometer"]
# [inline (always)]
pub const fn spectrometer (& self) -> & SPECTROMETER { & self . spectrometer } } # [doc = "product_id (r) register accessor: product_id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id`]
module"]
pub type PRODUCT_ID = crate :: Reg < product_id :: PRODUCT_ID_SPEC > ; # [doc = "product_id"]
pub mod product_id { # [doc = "Register `product_id` reader"]
pub type R = crate :: R < PRODUCT_ID_SPEC > ; # [doc = "Field `product_id` reader - product_id"]
pub type PRODUCT_ID_R = crate :: FieldReader < u32 > ; impl R { # [doc = "Bits 0:31 - product_id"]
# [inline (always)]
pub fn product_id (& self) -> PRODUCT_ID_R { PRODUCT_ID_R :: new (self . bits) } } # [doc = "product_id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRODUCT_ID_SPEC ; impl crate :: RegisterSpec for PRODUCT_ID_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`product_id::R`](R) reader structure"]
impl crate :: Readable for PRODUCT_ID_SPEC { } } # [doc = "version (r) register accessor: version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
pub type VERSION = crate :: Reg < version :: VERSION_SPEC > ; # [doc = "version"]
pub mod version { # [doc = "Register `version` reader"]
pub type R = crate :: R < VERSION_SPEC > ; # [doc = "Field `bugfix` reader - bugfix"]
pub type BUGFIX_R = crate :: FieldReader ; # [doc = "Field `minor` reader - minor"]
pub type MINOR_R = crate :: FieldReader ; # [doc = "Field `major` reader - major"]
pub type MAJOR_R = crate :: FieldReader ; # [doc = "Field `platform` reader - platform"]
pub type PLATFORM_R = crate :: FieldReader ; impl R { # [doc = "Bits 0:7 - bugfix"]
# [inline (always)]
pub fn bugfix (& self) -> BUGFIX_R { BUGFIX_R :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 8:15 - minor"]
# [inline (always)]
pub fn minor (& self) -> MINOR_R { MINOR_R :: new (((self . bits >> 8) & 0xff) as u8) } # [doc = "Bits 16:23 - major"]
# [inline (always)]
pub fn major (& self) -> MAJOR_R { MAJOR_R :: new (((self . bits >> 16) & 0xff) as u8) } # [doc = "Bits 24:31 - platform"]
# [inline (always)]
pub fn platform (& self) -> PLATFORM_R { PLATFORM_R :: new (((self . bits >> 24) & 0xff) as u8) } } # [doc = "version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC ; impl crate :: RegisterSpec for VERSION_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate :: Readable for VERSION_SPEC { } } # [doc = "control (rw) register accessor: control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate :: Reg < control :: CONTROL_SPEC > ; # [doc = "control"]
pub mod control { # [doc = "Register `control` reader"]
pub type R = crate :: R < CONTROL_SPEC > ; # [doc = "Register `control` writer"]
pub type W = crate :: W < CONTROL_SPEC > ; # [doc = "Field `sdr_reset` reader - sdr_reset"]
pub type SDR_RESET_R = crate :: BitReader ; # [doc = "Field `sdr_reset` writer - sdr_reset"]
pub type SDR_RESET_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bit 0 - sdr_reset"]
# [inline (always)]
pub fn sdr_reset (& self) -> SDR_RESET_R { SDR_RESET_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - sdr_reset"]
# [inline (always)]
# [must_use]
pub fn sdr_reset (& mut self) -> SDR_RESET_W < CONTROL_SPEC > { SDR_RESET_W :: new (self , 0) } # [doc = r" Writes raw bits to the register."]
# [doc = r""]
# [doc = r" # Safety"]
# [doc = r""]
# [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
# [inline (always)]
pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC ; impl crate :: RegisterSpec for CONTROL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate :: Readable for CONTROL_SPEC { } # [doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate :: Writable for CONTROL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } } # [doc = "interrupts (r) register accessor: interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupts`]
module"]
pub type INTERRUPTS = crate :: Reg < interrupts :: INTERRUPTS_SPEC > ; # [doc = "interrupts"]
pub mod interrupts { # [doc = "Register `interrupts` reader"]
pub type R = crate :: R < INTERRUPTS_SPEC > ; # [doc = "Field `spectrometer` reader - spectrometer"]
pub type SPECTROMETER_R = crate :: BitReader ; # [doc = "Field `recorder` reader - recorder"]
pub type RECORDER_R = crate :: BitReader ; impl R { # [doc = "Bit 0 - spectrometer"]
# [inline (always)]
pub fn spectrometer (& self) -> SPECTROMETER_R { SPECTROMETER_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - recorder"]
# [inline (always)]
pub fn recorder (& self) -> RECORDER_R { RECORDER_R :: new (((self . bits >> 1) & 1) != 0) } } # [doc = "interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPTS_SPEC ; impl crate :: RegisterSpec for INTERRUPTS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`interrupts::R`](R) reader structure"]
impl crate :: Readable for INTERRUPTS_SPEC { } } # [doc = "recorder_control (rw) register accessor: recorder_control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recorder_control::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`recorder_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recorder_control`]
module"]
pub type RECORDER_CONTROL = crate :: Reg < recorder_control :: RECORDER_CONTROL_SPEC > ; # [doc = "recorder_control"]
pub mod recorder_control { # [doc = "Register `recorder_control` reader"]
pub type R = crate :: R < RECORDER_CONTROL_SPEC > ; # [doc = "Register `recorder_control` writer"]
pub type W = crate :: W < RECORDER_CONTROL_SPEC > ; # [doc = "Field `start` writer - start"]
pub type START_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `stop` writer - stop"]
pub type STOP_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `mode_8bit` reader - mode_8bit"]
pub type MODE_8BIT_R = crate :: BitReader ; # [doc = "Field `mode_8bit` writer - mode_8bit"]
pub type MODE_8BIT_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; # [doc = "Field `dropped_samples` reader - dropped_samples"]
pub type DROPPED_SAMPLES_R = crate :: BitReader ; impl R { # [doc = "Bit 2 - mode_8bit"]
# [inline (always)]
pub fn mode_8bit (& self) -> MODE_8BIT_R { MODE_8BIT_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - dropped_samples"]
# [inline (always)]
pub fn dropped_samples (& self) -> DROPPED_SAMPLES_R { DROPPED_SAMPLES_R :: new (((self . bits >> 3) & 1) != 0) } } impl W { # [doc = "Bit 0 - start"]
# [inline (always)]
# [must_use]
pub fn start (& mut self) -> START_W < RECORDER_CONTROL_SPEC > { START_W :: new (self , 0) } # [doc = "Bit 1 - stop"]
# [inline (always)]
# [must_use]
pub fn stop (& mut self) -> STOP_W < RECORDER_CONTROL_SPEC > { STOP_W :: new (self , 1) } # [doc = "Bit 2 - mode_8bit"]
# [inline (always)]
# [must_use]
pub fn mode_8bit (& mut self) -> MODE_8BIT_W < RECORDER_CONTROL_SPEC > { MODE_8BIT_W :: new (self , 2) } # [doc = r" Writes raw bits to the register."]
# [doc = r""]
# [doc = r" # Safety"]
# [doc = r""]
# [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
# [inline (always)]
pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "recorder_control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recorder_control::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`recorder_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECORDER_CONTROL_SPEC ; impl crate :: RegisterSpec for RECORDER_CONTROL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`recorder_control::R`](R) reader structure"]
impl crate :: Readable for RECORDER_CONTROL_SPEC { } # [doc = "`write(|w| ..)` method takes [`recorder_control::W`](W) writer structure"]
impl crate :: Writable for RECORDER_CONTROL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } } # [doc = "recorder_next_address (r) register accessor: recorder_next_address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recorder_next_address::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@recorder_next_address`]
module"]
pub type RECORDER_NEXT_ADDRESS = crate :: Reg < recorder_next_address :: RECORDER_NEXT_ADDRESS_SPEC > ; # [doc = "recorder_next_address"]
pub mod recorder_next_address { # [doc = "Register `recorder_next_address` reader"]
pub type R = crate :: R < RECORDER_NEXT_ADDRESS_SPEC > ; # [doc = "Field `next_address` reader - next_address"]
pub type NEXT_ADDRESS_R = crate :: FieldReader < u32 > ; impl R { # [doc = "Bits 0:31 - next_address"]
# [inline (always)]
pub fn next_address (& self) -> NEXT_ADDRESS_R { NEXT_ADDRESS_R :: new (self . bits) } } # [doc = "recorder_next_address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`recorder_next_address::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECORDER_NEXT_ADDRESS_SPEC ; impl crate :: RegisterSpec for RECORDER_NEXT_ADDRESS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`recorder_next_address::R`](R) reader structure"]
impl crate :: Readable for RECORDER_NEXT_ADDRESS_SPEC { } } # [doc = "spectrometer (rw) register accessor: spectrometer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectrometer::R`].  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectrometer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spectrometer`]
module"]
pub type SPECTROMETER = crate :: Reg < spectrometer :: SPECTROMETER_SPEC > ; # [doc = "spectrometer"]
pub mod spectrometer { # [doc = "Register `spectrometer` reader"]
pub type R = crate :: R < SPECTROMETER_SPEC > ; # [doc = "Register `spectrometer` writer"]
pub type W = crate :: W < SPECTROMETER_SPEC > ; # [doc = "Field `num_integrations` reader - num_integrations"]
pub type NUM_INTEGRATIONS_R = crate :: FieldReader < u16 > ; # [doc = "Field `num_integrations` writer - num_integrations"]
pub type NUM_INTEGRATIONS_W < 'a , REG > = crate :: FieldWriter < 'a , REG , 10 , u16 > ; # [doc = "Field `last_buffer` reader - last_buffer"]
pub type LAST_BUFFER_R = crate :: FieldReader ; # [doc = "Field `peak_detect` reader - peak_detect"]
pub type PEAK_DETECT_R = crate :: BitReader ; # [doc = "Field `peak_detect` writer - peak_detect"]
pub type PEAK_DETECT_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bits 0:9 - num_integrations"]
# [inline (always)]
pub fn num_integrations (& self) -> NUM_INTEGRATIONS_R { NUM_INTEGRATIONS_R :: new ((self . bits & 0x03ff) as u16) } # [doc = "Bits 10:12 - last_buffer"]
# [inline (always)]
pub fn last_buffer (& self) -> LAST_BUFFER_R { LAST_BUFFER_R :: new (((self . bits >> 10) & 7) as u8) } # [doc = "Bit 13 - peak_detect"]
# [inline (always)]
pub fn peak_detect (& self) -> PEAK_DETECT_R { PEAK_DETECT_R :: new (((self . bits >> 13) & 1) != 0) } } impl W { # [doc = "Bits 0:9 - num_integrations"]
# [inline (always)]
# [must_use]
pub fn num_integrations (& mut self) -> NUM_INTEGRATIONS_W < SPECTROMETER_SPEC > { NUM_INTEGRATIONS_W :: new (self , 0) } # [doc = "Bit 13 - peak_detect"]
# [inline (always)]
# [must_use]
pub fn peak_detect (& mut self) -> PEAK_DETECT_W < SPECTROMETER_SPEC > { PEAK_DETECT_W :: new (self , 13) } # [doc = r" Writes raw bits to the register."]
# [doc = r""]
# [doc = r" # Safety"]
# [doc = r""]
# [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
# [inline (always)]
pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "spectrometer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spectrometer::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spectrometer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPECTROMETER_SPEC ; impl crate :: RegisterSpec for SPECTROMETER_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`spectrometer::R`](R) reader structure"]
impl crate :: Readable for SPECTROMETER_SPEC { } # [doc = "`write(|w| ..)` method takes [`spectrometer::W`](W) writer structure"]
impl crate :: Writable for SPECTROMETER_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } } } # [no_mangle]
static mut DEVICE_PERIPHERALS : bool = false ; # [doc = r" All the peripherals."]
# [allow (non_snake_case)]
pub struct Peripherals { # [doc = "MAIA_SDR"]
pub MAIA_SDR : MAIA_SDR , } impl Peripherals { # [doc = r" Returns all the peripherals *once*."]
# [cfg (feature = "critical-section")]
# [inline]
pub fn take () -> Option < Self > { critical_section :: with (| _ | { if unsafe { DEVICE_PERIPHERALS } { return None } Some (unsafe { Peripherals :: steal () }) }) } # [doc = r" Unchecked version of `Peripherals::take`."]
# [doc = r""]
# [doc = r" # Safety"]
# [doc = r""]
# [doc = r" Each of the returned peripherals must be used at most once."]
# [inline]
pub unsafe fn steal () -> Self { DEVICE_PERIPHERALS = true ; Peripherals { MAIA_SDR : MAIA_SDR { _marker : PhantomData } , } } }