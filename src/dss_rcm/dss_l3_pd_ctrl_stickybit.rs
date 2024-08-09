#[doc = "Register `DSS_L3_PD_CTRL_STICKYBIT` reader"]
pub type R = crate::R<DssL3PdCtrlStickybitSpec>;
#[doc = "Register `DSS_L3_PD_CTRL_STICKYBIT` writer"]
pub type W = crate::W<DssL3PdCtrlStickybitSpec>;
#[doc = "Field `set` reader - 2:0\\]
Sticky bit for DSS L3 PD CTRL. Write 3'b111 to lock the configuration of DSS_L3_BANK*_PD_CTRL. Once this field is writen, there is no impact of changing the value of aonin and agoodin fields in DSS_L3_BANK*_PD_CTRL registers"]
pub type SetR = crate::FieldReader;
#[doc = "Field `set` writer - 2:0\\]
Sticky bit for DSS L3 PD CTRL. Write 3'b111 to lock the configuration of DSS_L3_BANK*_PD_CTRL. Once this field is writen, there is no impact of changing the value of aonin and agoodin fields in DSS_L3_BANK*_PD_CTRL registers"]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Sticky bit for DSS L3 PD CTRL. Write 3'b111 to lock the configuration of DSS_L3_BANK*_PD_CTRL. Once this field is writen, there is no impact of changing the value of aonin and agoodin fields in DSS_L3_BANK*_PD_CTRL registers"]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Sticky bit for DSS L3 PD CTRL. Write 3'b111 to lock the configuration of DSS_L3_BANK*_PD_CTRL. Once this field is writen, there is no impact of changing the value of aonin and agoodin fields in DSS_L3_BANK*_PD_CTRL registers"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<DssL3PdCtrlStickybitSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DSS_L3_PD_CTRL_STICKYBIT\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3_pd_ctrl_stickybit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3_pd_ctrl_stickybit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3PdCtrlStickybitSpec;
impl crate::RegisterSpec for DssL3PdCtrlStickybitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3_pd_ctrl_stickybit::R`](R) reader structure"]
impl crate::Readable for DssL3PdCtrlStickybitSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3_pd_ctrl_stickybit::W`](W) writer structure"]
impl crate::Writable for DssL3PdCtrlStickybitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3_PD_CTRL_STICKYBIT to value 0"]
impl crate::Resettable for DssL3PdCtrlStickybitSpec {
    const RESET_VALUE: u32 = 0;
}
