#[doc = "Register `PID` reader"]
pub type R = crate::R<PidSpec>;
#[doc = "Register `PID` writer"]
pub type W = crate::W<PidSpec>;
#[doc = "Field `MINOR` reader - 5:0\\]
Minor Revision"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:0\\]
Minor Revision"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MAJOR` reader - 10:8\\]
Major Revision"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 10:8\\]
Major Revision"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTL` reader - 15:11\\]
RTL Version"]
pub type RtlR = crate::FieldReader;
#[doc = "Field `RTL` writer - 15:11\\]
RTL Version"]
pub type RtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Function indicates a software compatible module family."]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Function indicates a software compatible module family."]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RES1` reader - 29:28\\]
RESERVE FIELD"]
pub type Res1R = crate::FieldReader;
#[doc = "Field `RES1` writer - 29:28\\]
RESERVE FIELD"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version"]
    #[inline(always)]
    pub fn rtl(&self) -> RtlR {
        RtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family."]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision"]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<PidSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision field: Not used on this version of EDMA."]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<PidSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<PidSpec> {
        MajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version"]
    #[inline(always)]
    #[must_use]
    pub fn rtl(&mut self) -> RtlW<PidSpec> {
        RtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<PidSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<PidSpec> {
        Res1W::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
PID Scheme: Used to distinguish between old ID scheme and current. Spare bit to encode future schemes EDMA uses 'new scheme' indicated with value of 0x1."]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<PidSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PidSpec;
impl crate::RegisterSpec for PidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid::R`](R) reader structure"]
impl crate::Readable for PidSpec {}
#[doc = "`write(|w| ..)` method takes [`pid::W`](W) writer structure"]
impl crate::Writable for PidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PidSpec {
    const RESET_VALUE: u32 = 0;
}
