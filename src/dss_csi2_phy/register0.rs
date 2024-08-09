#[doc = "Register `REGISTER0` reader"]
pub type R = crate::R<Register0Spec>;
#[doc = "Register `REGISTER0` writer"]
pub type W = crate::W<Register0Spec>;
#[doc = "Field `REG_THSEXIT` reader - 7:0\\]
REG_THSEXIT timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHy spec : > 100 ns Actual value seen on line : N = REG_THSEXIT = THSEXIT timer + analog delay &amp; slew on LP signals = \\[ceil\\[N/4\\]
* 4 \\]
* DDR_CLOCK_PERIOD - (~3 ns - 45 ns ) PROGRAMMED VALUE = ceil ( 145 ns/ DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegThsexitR = crate::FieldReader;
#[doc = "Field `REG_THSEXIT` writer - 7:0\\]
REG_THSEXIT timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHy spec : > 100 ns Actual value seen on line : N = REG_THSEXIT = THSEXIT timer + analog delay &amp; slew on LP signals = \\[ceil\\[N/4\\]
* 4 \\]
* DDR_CLOCK_PERIOD - (~3 ns - 45 ns ) PROGRAMMED VALUE = ceil ( 145 ns/ DDR Clock Period) Default value is programmed for 400 MHz"]
pub type RegThsexitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_THSTRAIL` reader - 15:8\\]
REG_THSTRAIL timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec : > 60ns + 4UI Actual value seen on line : N = REG_THSTRAIL = \\[ceil \\[(N+3)/4\\]
* 4 - 2.75\\]
* DDR_Clock_period +(~ 0ns - 5ns) PROGRAMMED VALUE = ceil ( 60 ns / DDR Clock Period ) + 5 Default value is programmed for 400 MHz"]
pub type RegThstrailR = crate::FieldReader;
#[doc = "Field `REG_THSTRAIL` writer - 15:8\\]
REG_THSTRAIL timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec : > 60ns + 4UI Actual value seen on line : N = REG_THSTRAIL = \\[ceil \\[(N+3)/4\\]
* 4 - 2.75\\]
* DDR_Clock_period +(~ 0ns - 5ns) PROGRAMMED VALUE = ceil ( 60 ns / DDR Clock Period ) + 5 Default value is programmed for 400 MHz"]
pub type RegThstrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_THSPRPR_THSZERO` reader - 23:16\\]
REG_THSPREPARE_THSZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec: > 145ns + 10 UI Actual value seen on line N= REG_THSPREPARE_THSZERO - REG_THSPREPARE M=REG_THSPREPARE = \\[ceil \\[(N+3)/4\\]
* 4 + ceil \\[M/4\\]
* 4 + 3\\]
* DDR_Clock_Period+\\[~-29ns - 0ns\\]
PROGRAMMED VALUE = ceil ( 175 ns/ DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
pub type RegThsprprThszeroR = crate::FieldReader;
#[doc = "Field `REG_THSPRPR_THSZERO` writer - 23:16\\]
REG_THSPREPARE_THSZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec: > 145ns + 10 UI Actual value seen on line N= REG_THSPREPARE_THSZERO - REG_THSPREPARE M=REG_THSPREPARE = \\[ceil \\[(N+3)/4\\]
* 4 + ceil \\[M/4\\]
* 4 + 3\\]
* DDR_Clock_Period+\\[~-29ns - 0ns\\]
PROGRAMMED VALUE = ceil ( 175 ns/ DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
pub type RegThsprprThszeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_THSPREPARE` reader - 31:24\\]
REG_THSPREPARE timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec:40ns+4UI-85ns+6UI Auctual value seen on line : = REG_THSPREPARE timer + analog delay &amp; slew on signals = REG_THSPREPARE +(-26.5ns -- +4 ns ) PROGRAMMED VALUE = ceil( 70 ns / DDR Clock Period) + 2 Default value is programmed for 400 MHz"]
pub type RegThsprepareR = crate::FieldReader;
#[doc = "Field `REG_THSPREPARE` writer - 31:24\\]
REG_THSPREPARE timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec:40ns+4UI-85ns+6UI Auctual value seen on line : = REG_THSPREPARE timer + analog delay &amp; slew on signals = REG_THSPREPARE +(-26.5ns -- +4 ns ) PROGRAMMED VALUE = ceil( 70 ns / DDR Clock Period) + 2 Default value is programmed for 400 MHz"]
pub type RegThsprepareW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
REG_THSEXIT timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHy spec : > 100 ns Actual value seen on line : N = REG_THSEXIT = THSEXIT timer + analog delay &amp; slew on LP signals = \\[ceil\\[N/4\\]
* 4 \\]
* DDR_CLOCK_PERIOD - (~3 ns - 45 ns ) PROGRAMMED VALUE = ceil ( 145 ns/ DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_thsexit(&self) -> RegThsexitR {
        RegThsexitR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
REG_THSTRAIL timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec : > 60ns + 4UI Actual value seen on line : N = REG_THSTRAIL = \\[ceil \\[(N+3)/4\\]
* 4 - 2.75\\]
* DDR_Clock_period +(~ 0ns - 5ns) PROGRAMMED VALUE = ceil ( 60 ns / DDR Clock Period ) + 5 Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_thstrail(&self) -> RegThstrailR {
        RegThstrailR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
REG_THSPREPARE_THSZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec: > 145ns + 10 UI Actual value seen on line N= REG_THSPREPARE_THSZERO - REG_THSPREPARE M=REG_THSPREPARE = \\[ceil \\[(N+3)/4\\]
* 4 + ceil \\[M/4\\]
* 4 + 3\\]
* DDR_Clock_Period+\\[~-29ns - 0ns\\]
PROGRAMMED VALUE = ceil ( 175 ns/ DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_thsprpr_thszero(&self) -> RegThsprprThszeroR {
        RegThsprprThszeroR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
REG_THSPREPARE timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec:40ns+4UI-85ns+6UI Auctual value seen on line : = REG_THSPREPARE timer + analog delay &amp; slew on signals = REG_THSPREPARE +(-26.5ns -- +4 ns ) PROGRAMMED VALUE = ceil( 70 ns / DDR Clock Period) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    pub fn reg_thsprepare(&self) -> RegThsprepareR {
        RegThsprepareR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
REG_THSEXIT timing parameter in multiples of DDR clock frequency. DDR clock = CLKIN4DDR/4. D-PHy spec : > 100 ns Actual value seen on line : N = REG_THSEXIT = THSEXIT timer + analog delay &amp; slew on LP signals = \\[ceil\\[N/4\\]
* 4 \\]
* DDR_CLOCK_PERIOD - (~3 ns - 45 ns ) PROGRAMMED VALUE = ceil ( 145 ns/ DDR Clock Period) Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_thsexit(&mut self) -> RegThsexitW<Register0Spec> {
        RegThsexitW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
REG_THSTRAIL timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec : > 60ns + 4UI Actual value seen on line : N = REG_THSTRAIL = \\[ceil \\[(N+3)/4\\]
* 4 - 2.75\\]
* DDR_Clock_period +(~ 0ns - 5ns) PROGRAMMED VALUE = ceil ( 60 ns / DDR Clock Period ) + 5 Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_thstrail(&mut self) -> RegThstrailW<Register0Spec> {
        RegThstrailW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
REG_THSPREPARE_THSZERO timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec: > 145ns + 10 UI Actual value seen on line N= REG_THSPREPARE_THSZERO - REG_THSPREPARE M=REG_THSPREPARE = \\[ceil \\[(N+3)/4\\]
* 4 + ceil \\[M/4\\]
* 4 + 3\\]
* DDR_Clock_Period+\\[~-29ns - 0ns\\]
PROGRAMMED VALUE = ceil ( 175 ns/ DDR Clock Period ) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_thsprpr_thszero(&mut self) -> RegThsprprThszeroW<Register0Spec> {
        RegThsprprThszeroW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
REG_THSPREPARE timing parameter in multiples of DDR clock period. DDR clock = CLKIN4DDR/4. D-PHY spec:40ns+4UI-85ns+6UI Auctual value seen on line : = REG_THSPREPARE timer + analog delay &amp; slew on signals = REG_THSPREPARE +(-26.5ns -- +4 ns ) PROGRAMMED VALUE = ceil( 70 ns / DDR Clock Period) + 2 Default value is programmed for 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn reg_thsprepare(&mut self) -> RegThsprepareW<Register0Spec> {
        RegThsprepareW::new(self, 24)
    }
}
#[doc = "First Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register0Spec;
impl crate::RegisterSpec for Register0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register0::R`](R) reader structure"]
impl crate::Readable for Register0Spec {}
#[doc = "`write(|w| ..)` method takes [`register0::W`](W) writer structure"]
impl crate::Writable for Register0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER0 to value 0"]
impl crate::Resettable for Register0Spec {
    const RESET_VALUE: u32 = 0;
}
