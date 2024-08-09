#[doc = "Register `REGISTER1` reader"]
pub type R = crate::R<Register1Spec>;
#[doc = "Register `REGISTER1` writer"]
pub type W = crate::W<Register1Spec>;
#[doc = "Field `REG_TCLKZERO` reader - 7:0\\]
REG_TCLKZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec :\\[REG_TCLKPREPARE + REG_TCLKZERO\\]
> 300 ns Derived spec for REG_TCLKERO (Min REG_TCLKPREPARE = 38 ns) REG_TCLKZERO > 262 ns Actual value seen on line : N = REG_TCLKZERO M = REG_TCLKPREPARE = \\[ceil \\[(N+3)/4\\]
*4 + ceil (M/4) * 4 - M + 2\\]
* DDR_Clock_Period + \\[~0ns - 5ns\\]
PROGRA MMED VALUE = ceil (265 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegTclkzeroR = crate::FieldReader;
#[doc = "Field `REG_TCLKZERO` writer - 7:0\\]
REG_TCLKZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec :\\[REG_TCLKPREPARE + REG_TCLKZERO\\]
> 300 ns Derived spec for REG_TCLKERO (Min REG_TCLKPREPARE = 38 ns) REG_TCLKZERO > 262 ns Actual value seen on line : N = REG_TCLKZERO M = REG_TCLKPREPARE = \\[ceil \\[(N+3)/4\\]
*4 + ceil (M/4) * 4 - M + 2\\]
* DDR_Clock_Period + \\[~0ns - 5ns\\]
PROGRA MMED VALUE = ceil (265 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegTclkzeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TCLKTRAIL` reader - 15:8\\]
REG_TCLKTRAIL timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHY spec : 60 ns PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period) Actual value seen on line: N = REG_TCLKTRAIL = \\[ceil\\[(N+3)/4\\]
*4 - 1.5\\]
* DDR_Clock_Period + (~0 ns - 5 ns) PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
pub type RegTclktrailR = crate::FieldReader;
#[doc = "Field `REG_TCLKTRAIL` writer - 15:8\\]
REG_TCLKTRAIL timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHY spec : 60 ns PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period) Actual value seen on line: N = REG_TCLKTRAIL = \\[ceil\\[(N+3)/4\\]
*4 - 1.5\\]
* DDR_Clock_Period + (~0 ns - 5 ns) PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
pub type RegTclktrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TLPXBY2` reader - 20:16\\]
(TLPX)/2 timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. PROGRAMMED VALUE = ceil (25 ns / DDR Clock Period ) Default value is programmed for 400 MHz Note : TLPX is used to define the length of LP-01 state in HS start of Transmision sequences on clock and data lanes. For all other purposes TLPX is defined by the period of TXLPESC"]
pub type RegTlpxby2R = crate::FieldReader;
#[doc = "Field `REG_TLPXBY2` writer - 20:16\\]
(TLPX)/2 timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. PROGRAMMED VALUE = ceil (25 ns / DDR Clock Period ) Default value is programmed for 400 MHz Note : TLPX is used to define the length of LP-01 state in HS start of Transmision sequences on clock and data lanes. For all other purposes TLPX is defined by the period of TXLPESC"]
pub type RegTlpxby2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EMPTY` reader - 23:21\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 23:21\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TTAGET` reader - 26:24\\]
TTA-GET timing in terms of number of TXCLKESC clocks 000 == 3 cycles 001 == 4 cycles 010 == 5 cycles 011 == 6 cycles 100 == 7 cycles 101 == 8 cycles 110 == 9 cycles 111 == 10 cycles Default value: 5 cycles"]
pub type RegTtagetR = crate::FieldReader;
#[doc = "Field `REG_TTAGET` writer - 26:24\\]
TTA-GET timing in terms of number of TXCLKESC clocks 000 == 3 cycles 001 == 4 cycles 010 == 5 cycles 011 == 6 cycles 100 == 7 cycles 101 == 8 cycles 110 == 9 cycles 111 == 10 cycles Default value: 5 cycles"]
pub type RegTtagetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_TTASURE` reader - 28:27\\]
TTA-SURE timing in terms of number of TXCLKESC clocks 00 == 2 cycles 01 == Reserved 10 == 3 cycles 11 == 4 cycles Default value: 2 cycles"]
pub type RegTtasureR = crate::FieldReader;
#[doc = "Field `REG_TTASURE` writer - 28:27\\]
TTA-SURE timing in terms of number of TXCLKESC clocks 00 == 2 cycles 01 == Reserved 10 == 3 cycles 11 == 4 cycles Default value: 2 cycles"]
pub type RegTtasureW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TTAGO` reader - 31:29\\]
TTA-GO timing in terms of number of TXCLKESC clocks 000 == 2 cycles 001 == 3 cycles 010 == 4 cycles 011 == 5 cycles 100 == 6 cycles 101 == 7 cycles 110 == 8 cycles 111 == 9 cycles Default value: 4 cycles"]
pub type RegTtagoR = crate::FieldReader;
#[doc = "Field `REG_TTAGO` writer - 31:29\\]
TTA-GO timing in terms of number of TXCLKESC clocks 000 == 2 cycles 001 == 3 cycles 010 == 4 cycles 011 == 5 cycles 100 == 6 cycles 101 == 7 cycles 110 == 8 cycles 111 == 9 cycles Default value: 4 cycles"]
pub type RegTtagoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
REG_TCLKZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec :\\[REG_TCLKPREPARE + REG_TCLKZERO\\]
> 300 ns Derived spec for REG_TCLKERO (Min REG_TCLKPREPARE = 38 ns) REG_TCLKZERO > 262 ns Actual value seen on line : N = REG_TCLKZERO M = REG_TCLKPREPARE = \\[ceil \\[(N+3)/4\\]
*4 + ceil (M/4) * 4 - M + 2\\]
* DDR_Clock_Period + \\[~0ns - 5ns\\]
PROGRA MMED VALUE = ceil (265 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_tclkzero(&self) -> RegTclkzeroR {
        RegTclkzeroR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
REG_TCLKTRAIL timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHY spec : 60 ns PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period) Actual value seen on line: N = REG_TCLKTRAIL = \\[ceil\\[(N+3)/4\\]
*4 - 1.5\\]
* DDR_Clock_Period + (~0 ns - 5 ns) PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_tclktrail(&self) -> RegTclktrailR {
        RegTclktrailR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
(TLPX)/2 timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. PROGRAMMED VALUE = ceil (25 ns / DDR Clock Period ) Default value is programmed for 400 MHz Note : TLPX is used to define the length of LP-01 state in HS start of Transmision sequences on clock and data lanes. For all other purposes TLPX is defined by the period of TXLPESC"]
    #[inline(always)]
    pub fn reg_tlpxby2(&self) -> RegTlpxby2R {
        RegTlpxby2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
TTA-GET timing in terms of number of TXCLKESC clocks 000 == 3 cycles 001 == 4 cycles 010 == 5 cycles 011 == 6 cycles 100 == 7 cycles 101 == 8 cycles 110 == 9 cycles 111 == 10 cycles Default value: 5 cycles"]
    #[inline(always)]
    pub fn reg_ttaget(&self) -> RegTtagetR {
        RegTtagetR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
TTA-SURE timing in terms of number of TXCLKESC clocks 00 == 2 cycles 01 == Reserved 10 == 3 cycles 11 == 4 cycles Default value: 2 cycles"]
    #[inline(always)]
    pub fn reg_ttasure(&self) -> RegTtasureR {
        RegTtasureR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
TTA-GO timing in terms of number of TXCLKESC clocks 000 == 2 cycles 001 == 3 cycles 010 == 4 cycles 011 == 5 cycles 100 == 6 cycles 101 == 7 cycles 110 == 8 cycles 111 == 9 cycles Default value: 4 cycles"]
    #[inline(always)]
    pub fn reg_ttago(&self) -> RegTtagoR {
        RegTtagoR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
REG_TCLKZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec :\\[REG_TCLKPREPARE + REG_TCLKZERO\\]
> 300 ns Derived spec for REG_TCLKERO (Min REG_TCLKPREPARE = 38 ns) REG_TCLKZERO > 262 ns Actual value seen on line : N = REG_TCLKZERO M = REG_TCLKPREPARE = \\[ceil \\[(N+3)/4\\]
*4 + ceil (M/4) * 4 - M + 2\\]
* DDR_Clock_Period + \\[~0ns - 5ns\\]
PROGRA MMED VALUE = ceil (265 ns / DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tclkzero(&mut self) -> RegTclkzeroW<Register1Spec> {
        RegTclkzeroW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
REG_TCLKTRAIL timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHY spec : 60 ns PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period) Actual value seen on line: N = REG_TCLKTRAIL = \\[ceil\\[(N+3)/4\\]
*4 - 1.5\\]
* DDR_Clock_Period + (~0 ns - 5 ns) PROGRAMMED VALUE = ceil (60 ns / DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tclktrail(&mut self) -> RegTclktrailW<Register1Spec> {
        RegTclktrailW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
(TLPX)/2 timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. PROGRAMMED VALUE = ceil (25 ns / DDR Clock Period ) Default value is programmed for 400 MHz Note : TLPX is used to define the length of LP-01 state in HS start of Transmision sequences on clock and data lanes. For all other purposes TLPX is defined by the period of TXLPESC"]
    #[inline(always)]
    #[must_use]
    pub fn reg_tlpxby2(&mut self) -> RegTlpxby2W<Register1Spec> {
        RegTlpxby2W::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register1Spec> {
        EmptyW::new(self, 21)
    }
    #[doc = "Bits 24:26 - 26:24\\]
TTA-GET timing in terms of number of TXCLKESC clocks 000 == 3 cycles 001 == 4 cycles 010 == 5 cycles 011 == 6 cycles 100 == 7 cycles 101 == 8 cycles 110 == 9 cycles 111 == 10 cycles Default value: 5 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ttaget(&mut self) -> RegTtagetW<Register1Spec> {
        RegTtagetW::new(self, 24)
    }
    #[doc = "Bits 27:28 - 28:27\\]
TTA-SURE timing in terms of number of TXCLKESC clocks 00 == 2 cycles 01 == Reserved 10 == 3 cycles 11 == 4 cycles Default value: 2 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ttasure(&mut self) -> RegTtasureW<Register1Spec> {
        RegTtasureW::new(self, 27)
    }
    #[doc = "Bits 29:31 - 31:29\\]
TTA-GO timing in terms of number of TXCLKESC clocks 000 == 2 cycles 001 == 3 cycles 010 == 4 cycles 011 == 5 cycles 100 == 6 cycles 101 == 7 cycles 110 == 8 cycles 111 == 9 cycles Default value: 4 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ttago(&mut self) -> RegTtagoW<Register1Spec> {
        RegTtagoW::new(self, 29)
    }
}
#[doc = "REGISTER1\n\nYou can [`read`](crate::Reg::read) this register and get [`register1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register1Spec;
impl crate::RegisterSpec for Register1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register1::R`](R) reader structure"]
impl crate::Readable for Register1Spec {}
#[doc = "`write(|w| ..)` method takes [`register1::W`](W) writer structure"]
impl crate::Writable for Register1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER1 to value 0"]
impl crate::Resettable for Register1Spec {
    const RESET_VALUE: u32 = 0;
}
