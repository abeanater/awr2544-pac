#[doc = "Register `DSS_L3_BANKB1_PD_CTRL` reader"]
pub type R = crate::R<DssL3Bankb1PdCtrlSpec>;
#[doc = "Register `DSS_L3_BANKB1_PD_CTRL` writer"]
pub type W = crate::W<DssL3Bankb1PdCtrlSpec>;
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
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SW Control for &lt;IP>_PD_CTRL Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<DssL3Bankb1PdCtrlSpec> {
        IsoW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL0"]
    #[inline(always)]
    #[must_use]
    pub fn aonin(&mut self) -> AoninW<DssL3Bankb1PdCtrlSpec> {
        AoninW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SW Control for &lt;IP>_PD_CTRL Memory Array Power up CRTL1"]
    #[inline(always)]
    #[must_use]
    pub fn agoodin(&mut self) -> AgoodinW<DssL3Bankb1PdCtrlSpec> {
        AgoodinW::new(self, 8)
    }
}
#[doc = "DSS_L3_BANKB1_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_bankb1_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_bankb1_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3Bankb1PdCtrlSpec;
impl crate::RegisterSpec for DssL3Bankb1PdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3_bankb1_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for DssL3Bankb1PdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3_bankb1_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for DssL3Bankb1PdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3_BANKB1_PD_CTRL to value 0"]
impl crate::Resettable for DssL3Bankb1PdCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
