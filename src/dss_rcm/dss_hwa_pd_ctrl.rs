#[doc = "Register `DSS_HWA_PD_CTRL` reader"]
pub type R = crate::R<DssHwaPdCtrlSpec>;
#[doc = "Register `DSS_HWA_PD_CTRL` writer"]
pub type W = crate::W<DssHwaPdCtrlSpec>;
#[doc = "Field `iso` reader - 2:0\\]
SW Control for &lt;IP>_PD_CTRL Isolation"]
pub type IsoR = crate::FieldReader;
#[doc = "Field `iso` writer - 2:0\\]
SW Control for &lt;IP>_PD_CTRL Isolation"]
pub type IsoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aonin` reader - 6:4\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AoninR = crate::FieldReader;
#[doc = "Field `aonin` writer - 6:4\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AoninW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `agoodin` reader - 10:8\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodinR = crate::FieldReader;
#[doc = "Field `agoodin` writer - 10:8\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ponin` reader - 14:12\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL0"]
pub type PoninR = crate::FieldReader;
#[doc = "Field `ponin` writer - 14:12\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL0"]
pub type PoninW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pgoodin` reader - 18:16\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL1"]
pub type PgoodinR = crate::FieldReader;
#[doc = "Field `pgoodin` writer - 18:16\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL1"]
pub type PgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
SW Control for &lt;IP>_PD_CTRL Isolation"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    pub fn aonin(&self) -> AoninR {
        AoninR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    pub fn agoodin(&self) -> AgoodinR {
        AgoodinR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL0"]
    #[inline(always)]
    pub fn ponin(&self) -> PoninR {
        PoninR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL1"]
    #[inline(always)]
    pub fn pgoodin(&self) -> PgoodinR {
        PgoodinR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SW Control for &lt;IP>_PD_CTRL Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<DssHwaPdCtrlSpec> {
        IsoW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn aonin(&mut self) -> AoninW<DssHwaPdCtrlSpec> {
        AoninW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn agoodin(&mut self) -> AgoodinW<DssHwaPdCtrlSpec> {
        AgoodinW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn ponin(&mut self) -> PoninW<DssHwaPdCtrlSpec> {
        PoninW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
SW Control for &lt;IP>_PD_CTRL Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn pgoodin(&mut self) -> PgoodinW<DssHwaPdCtrlSpec> {
        PgoodinW::new(self, 16)
    }
}
#[doc = "DSS_HWA_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssHwaPdCtrlSpec;
impl crate::RegisterSpec for DssHwaPdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_hwa_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for DssHwaPdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_hwa_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for DssHwaPdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_HWA_PD_CTRL to value 0"]
impl crate::Resettable for DssHwaPdCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
