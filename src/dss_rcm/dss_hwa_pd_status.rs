#[doc = "Register `DSS_HWA_PD_STATUS` reader"]
pub type R = crate::R<DssHwaPdStatusSpec>;
#[doc = "Register `DSS_HWA_PD_STATUS` writer"]
pub type W = crate::W<DssHwaPdStatusSpec>;
#[doc = "Field `aonout` reader - 0:0\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AonoutR = crate::BitReader;
#[doc = "Field `aonout` writer - 0:0\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agoodout` reader - 1:1\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodoutR = crate::BitReader;
#[doc = "Field `agoodout` writer - 1:1\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ponout` reader - 2:2\\]
Status for &lt;IP>_PD_CTRL Power up CRTL0"]
pub type PonoutR = crate::BitReader;
#[doc = "Field `ponout` writer - 2:2\\]
Status for &lt;IP>_PD_CTRL Power up CRTL0"]
pub type PonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pgoodout` reader - 3:3\\]
Status for &lt;IP>_PD_CTRL Power up CRTL1"]
pub type PgoodoutR = crate::BitReader;
#[doc = "Field `pgoodout` writer - 3:3\\]
Status for &lt;IP>_PD_CTRL Power up CRTL1"]
pub type PgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    pub fn aonout(&self) -> AonoutR {
        AonoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    pub fn agoodout(&self) -> AgoodoutR {
        AgoodoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status for &lt;IP>_PD_CTRL Power up CRTL0"]
    #[inline(always)]
    pub fn ponout(&self) -> PonoutR {
        PonoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status for &lt;IP>_PD_CTRL Power up CRTL1"]
    #[inline(always)]
    pub fn pgoodout(&self) -> PgoodoutR {
        PgoodoutR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn aonout(&mut self) -> AonoutW<DssHwaPdStatusSpec> {
        AonoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn agoodout(&mut self) -> AgoodoutW<DssHwaPdStatusSpec> {
        AgoodoutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status for &lt;IP>_PD_CTRL Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn ponout(&mut self) -> PonoutW<DssHwaPdStatusSpec> {
        PonoutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status for &lt;IP>_PD_CTRL Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn pgoodout(&mut self) -> PgoodoutW<DssHwaPdStatusSpec> {
        PgoodoutW::new(self, 3)
    }
}
#[doc = "DSS_HWA_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_hwa_pd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_hwa_pd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssHwaPdStatusSpec;
impl crate::RegisterSpec for DssHwaPdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_hwa_pd_status::R`](R) reader structure"]
impl crate::Readable for DssHwaPdStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_hwa_pd_status::W`](W) writer structure"]
impl crate::Writable for DssHwaPdStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_HWA_PD_STATUS to value 0"]
impl crate::Resettable for DssHwaPdStatusSpec {
    const RESET_VALUE: u32 = 0;
}
