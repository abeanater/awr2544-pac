#[doc = "Register `MSS_DBGSS_CTI_TRIG_SEL` reader"]
pub type R = crate::R<MssDbgssCtiTrigSelSpec>;
#[doc = "Register `MSS_DBGSS_CTI_TRIG_SEL` writer"]
pub type W = crate::W<MssDbgssCtiTrigSelSpec>;
#[doc = "Field `trig1` reader - 7:0\\]
Used for selecting the trigger source for 1st trigger of ONE_MCU_CTI"]
pub type Trig1R = crate::FieldReader;
#[doc = "Field `trig1` writer - 7:0\\]
Used for selecting the trigger source for 1st trigger of ONE_MCU_CTI"]
pub type Trig1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `trig2` reader - 15:8\\]
Used for selecting the trigger source for 2nd trigger of ONE_MCU_CTI"]
pub type Trig2R = crate::FieldReader;
#[doc = "Field `trig2` writer - 15:8\\]
Used for selecting the trigger source for 2nd trigger of ONE_MCU_CTI"]
pub type Trig2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `trig3` reader - 23:16\\]
Used for selecting the trigger source for 3rd trigger of ONE_MCU_CTI"]
pub type Trig3R = crate::FieldReader;
#[doc = "Field `trig3` writer - 23:16\\]
Used for selecting the trigger source for 3rd trigger of ONE_MCU_CTI"]
pub type Trig3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used for selecting the trigger source for 1st trigger of ONE_MCU_CTI"]
    #[inline(always)]
    pub fn trig1(&self) -> Trig1R {
        Trig1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Used for selecting the trigger source for 2nd trigger of ONE_MCU_CTI"]
    #[inline(always)]
    pub fn trig2(&self) -> Trig2R {
        Trig2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Used for selecting the trigger source for 3rd trigger of ONE_MCU_CTI"]
    #[inline(always)]
    pub fn trig3(&self) -> Trig3R {
        Trig3R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used for selecting the trigger source for 1st trigger of ONE_MCU_CTI"]
    #[inline(always)]
    #[must_use]
    pub fn trig1(&mut self) -> Trig1W<MssDbgssCtiTrigSelSpec> {
        Trig1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Used for selecting the trigger source for 2nd trigger of ONE_MCU_CTI"]
    #[inline(always)]
    #[must_use]
    pub fn trig2(&mut self) -> Trig2W<MssDbgssCtiTrigSelSpec> {
        Trig2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Used for selecting the trigger source for 3rd trigger of ONE_MCU_CTI"]
    #[inline(always)]
    #[must_use]
    pub fn trig3(&mut self) -> Trig3W<MssDbgssCtiTrigSelSpec> {
        Trig3W::new(self, 16)
    }
}
#[doc = "MSS_DBGSS_CTI_TRIG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_dbgss_cti_trig_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_dbgss_cti_trig_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssDbgssCtiTrigSelSpec;
impl crate::RegisterSpec for MssDbgssCtiTrigSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_dbgss_cti_trig_sel::R`](R) reader structure"]
impl crate::Readable for MssDbgssCtiTrigSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_dbgss_cti_trig_sel::W`](W) writer structure"]
impl crate::Writable for MssDbgssCtiTrigSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_DBGSS_CTI_TRIG_SEL to value 0"]
impl crate::Resettable for MssDbgssCtiTrigSelSpec {
    const RESET_VALUE: u32 = 0;
}
