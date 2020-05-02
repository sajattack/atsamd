#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `NVMP`"]
pub type NVMP_R = crate::R<u16, u16>;
#[doc = "Page Size\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSZ_A {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<PSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: PSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSZ`"]
pub type PSZ_R = crate::R<u8, PSZ_A>;
impl PSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSZ_A {
        match self.bits {
            0 => PSZ_A::_8,
            1 => PSZ_A::_16,
            2 => PSZ_A::_32,
            3 => PSZ_A::_64,
            4 => PSZ_A::_128,
            5 => PSZ_A::_256,
            6 => PSZ_A::_512,
            7 => PSZ_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PSZ_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSZ_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSZ_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSZ_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSZ_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSZ_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSZ_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSZ_A::_1024
    }
}
#[doc = "SmartEEPROM Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEE_A {
    #[doc = "10: 163840 bytes"]
    A = 10,
    #[doc = "9: 147456 bytes"]
    _9 = 9,
    #[doc = "8: 131072 bytes"]
    _8 = 8,
    #[doc = "7: 114688 bytes"]
    _7 = 7,
    #[doc = "6: 98304 bytes"]
    _6 = 6,
    #[doc = "5: 81920 bytes"]
    _5 = 5,
    #[doc = "4: 65536 bytes"]
    _4 = 4,
    #[doc = "3: 49152 bytes"]
    _3 = 3,
    #[doc = "2: 32768 bytes"]
    _2 = 2,
    #[doc = "1: 16384 bytes"]
    _1 = 1,
    #[doc = "0: 0 bytes"]
    _0 = 0,
}
impl From<SEE_A> for bool {
    #[inline(always)]
    fn from(variant: SEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEE`"]
pub type SEE_R = crate::R<bool, SEE_A>;
impl SEE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SEE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SEE_A::A),
            true => Val(SEE_A::_9),
            true => Val(SEE_A::_8),
            true => Val(SEE_A::_7),
            true => Val(SEE_A::_6),
            true => Val(SEE_A::_5),
            true => Val(SEE_A::_4),
            true => Val(SEE_A::_3),
            true => Val(SEE_A::_2),
            true => Val(SEE_A::_1),
            false => Val(SEE_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == SEE_A::A
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == SEE_A::_9
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SEE_A::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SEE_A::_7
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SEE_A::_6
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SEE_A::_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SEE_A::_4
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == SEE_A::_3
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SEE_A::_2
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEE_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEE_A::_0
    }
}
impl R {
    #[doc = "Bits 0:15 - NVM Pages"]
    #[inline(always)]
    pub fn nvmp(&self) -> NVMP_R {
        NVMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 31 - SmartEEPROM Supported"]
    #[inline(always)]
    pub fn see(&self) -> SEE_R {
        SEE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
