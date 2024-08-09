#[doc = "Register `CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_7` reader - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel7R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_7` writer - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_6` reader - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel6R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_6` writer - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_5` reader - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel5R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_5` writer - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_4` reader - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel4R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_4` writer - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_3` reader - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel3R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_3` writer - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_2` reader - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel2R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_2` writer - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL_1` reader - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel1R = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL_1` writer - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 7:7\\]
CPDMA FHost Channel 7 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 7:7\\]
CPDMA FHost Channel 7 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_7(&self) -> CpdmaFhostChannel7R {
        CpdmaFhostChannel7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_6(&self) -> CpdmaFhostChannel6R {
        CpdmaFhostChannel6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_5(&self) -> CpdmaFhostChannel5R {
        CpdmaFhostChannel5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_4(&self) -> CpdmaFhostChannel4R {
        CpdmaFhostChannel4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_3(&self) -> CpdmaFhostChannel3R {
        CpdmaFhostChannel3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_2(&self) -> CpdmaFhostChannel2R {
        CpdmaFhostChannel2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel_1(&self) -> CpdmaFhostChannel1R {
        CpdmaFhostChannel1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
CPDMA FHost Channel 7 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_7(
        &mut self,
    ) -> CpdmaFhostChannel7W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel7W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_6(
        &mut self,
    ) -> CpdmaFhostChannel6W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel6W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_5(
        &mut self,
    ) -> CpdmaFhostChannel5W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel5W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_4(
        &mut self,
    ) -> CpdmaFhostChannel4W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel4W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_3(
        &mut self,
    ) -> CpdmaFhostChannel3W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel3W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_2(
        &mut self,
    ) -> CpdmaFhostChannel2W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel2W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel_1(
        &mut self,
    ) -> CpdmaFhostChannel1W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannel1W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
CPDMA FHost Channel 7 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 7)
    }
}
#[doc = "CPDMA FHost Interrupt Masked SET\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswCpdmaIntCpdmaFhIntmaskSetRegSpec;
impl crate::RegisterSpec for CpswCpdmaIntCpdmaFhIntmaskSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::R`](R) reader structure"]
impl crate::Readable for CpswCpdmaIntCpdmaFhIntmaskSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_cpdma_int_cpdma_fh_intmask_set_reg::W`](W) writer structure"]
impl crate::Writable for CpswCpdmaIntCpdmaFhIntmaskSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG to value 0"]
impl crate::Resettable for CpswCpdmaIntCpdmaFhIntmaskSetRegSpec {
    const RESET_VALUE: u32 = 0;
}
