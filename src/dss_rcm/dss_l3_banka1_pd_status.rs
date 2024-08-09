#[doc = "Register `DSS_L3_BANKA1_PD_STATUS` reader"]
pub type R = crate::R<DssL3Banka1PdStatusSpec>;
#[doc = "Register `DSS_L3_BANKA1_PD_STATUS` writer"]
pub type W = crate::W<DssL3Banka1PdStatusSpec>;
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
#[doc = "Field `aonin` reader - 2:2\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AoninR = crate::BitReader;
#[doc = "Field `aonin` writer - 2:2\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
pub type AoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `agoodin` reader - 3:3\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodinR = crate::BitReader;
#[doc = "Field `agoodin` writer - 3:3\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
pub type AgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
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
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    pub fn aonin(&self) -> AoninR {
        AoninR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    pub fn agoodin(&self) -> AgoodinR {
        AgoodinR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn aonout(&mut self) -> AonoutW<DssL3Banka1PdStatusSpec> {
        AonoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn agoodout(&mut self) -> AgoodoutW<DssL3Banka1PdStatusSpec> {
        AgoodoutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn aonin(&mut self) -> AoninW<DssL3Banka1PdStatusSpec> {
        AoninW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status for sticky control &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn agoodin(&mut self) -> AgoodinW<DssL3Banka1PdStatusSpec> {
        AgoodinW::new(self, 3)
    }
}
#[doc = "DSS_L3_BANKA1_PD_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_banka1_pd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_banka1_pd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3Banka1PdStatusSpec;
impl crate::RegisterSpec for DssL3Banka1PdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3_banka1_pd_status::R`](R) reader structure"]
impl crate::Readable for DssL3Banka1PdStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3_banka1_pd_status::W`](W) writer structure"]
impl crate::Writable for DssL3Banka1PdStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3_BANKA1_PD_STATUS to value 0"]
impl crate::Resettable for DssL3Banka1PdStatusSpec {
    const RESET_VALUE: u32 = 0;
}
