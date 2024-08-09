#[doc = "Register `MSS_L2_BANKC_PD_CTRL` reader"]
pub type R = crate::R<MssL2BankcPdCtrlSpec>;
#[doc = "Register `MSS_L2_BANKC_PD_CTRL` writer"]
pub type W = crate::W<MssL2BankcPdCtrlSpec>;
#[doc = "Field `iso` reader - 2:0\\]
SW control for power signal 'ISO' for MSS_L2_BANKC"]
pub type IsoR = crate::FieldReader;
#[doc = "Field `iso` writer - 2:0\\]
SW control for power signal 'ISO' for MSS_L2_BANKC"]
pub type IsoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aonin` reader - 6:4\\]
SW control for power signal 'AONIN' for MSS_L2_BANKC"]
pub type AoninR = crate::FieldReader;
#[doc = "Field `aonin` writer - 6:4\\]
SW control for power signal 'AONIN' for MSS_L2_BANKC"]
pub type AoninW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `agoodin` reader - 10:8\\]
SW control for power signal 'AGOODIN' for MSS_L2_BANKC"]
pub type AgoodinR = crate::FieldReader;
#[doc = "Field `agoodin` writer - 10:8\\]
SW control for power signal 'AGOODIN' for MSS_L2_BANKC"]
pub type AgoodinW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
SW control for power signal 'ISO' for MSS_L2_BANKC"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
SW control for power signal 'AONIN' for MSS_L2_BANKC"]
    #[inline(always)]
    pub fn aonin(&self) -> AoninR {
        AoninR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SW control for power signal 'AGOODIN' for MSS_L2_BANKC"]
    #[inline(always)]
    pub fn agoodin(&self) -> AgoodinR {
        AgoodinR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SW control for power signal 'ISO' for MSS_L2_BANKC"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<MssL2BankcPdCtrlSpec> {
        IsoW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
SW control for power signal 'AONIN' for MSS_L2_BANKC"]
    #[inline(always)]
    #[must_use]
    pub fn aonin(&mut self) -> AoninW<MssL2BankcPdCtrlSpec> {
        AoninW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
SW control for power signal 'AGOODIN' for MSS_L2_BANKC"]
    #[inline(always)]
    #[must_use]
    pub fn agoodin(&mut self) -> AgoodinW<MssL2BankcPdCtrlSpec> {
        AgoodinW::new(self, 8)
    }
}
#[doc = "MSS_L2_BANKC_PD_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_bankc_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_bankc_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2BankcPdCtrlSpec;
impl crate::RegisterSpec for MssL2BankcPdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_bankc_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for MssL2BankcPdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_bankc_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for MssL2BankcPdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_BANKC_PD_CTRL to value 0"]
impl crate::Resettable for MssL2BankcPdCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
