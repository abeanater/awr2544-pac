#[doc = "Register `CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG` reader"]
pub type R = crate::R<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec>;
#[doc = "Register `CPSW_CPDMA_INT_CPDMA_FH_INTMASK_SET_REG` writer"]
pub type W = crate::W<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 0:0\\]
CPDMA FHost Channel 0 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPDMA_FHOST_CHANNEL` reader - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelR = crate::BitReader;
#[doc = "Field `CPDMA_FHOST_CHANNEL` writer - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
pub type CpdmaFhostChannelW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
    #[inline(always)]
    pub fn cpdma_fhost_channel(&self) -> CpdmaFhostChannelR {
        CpdmaFhostChannelR::new(((self.bits >> 6) & 1) != 0)
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
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPDMA FHost Channel 1 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CPDMA FHost Channel 2 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CPDMA FHost Channel 3 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CPDMA FHost Channel 4 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
CPDMA FHost Channel 5 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CPDMA FHost Channel 6 Interrupt Pending MASKED Set"]
    #[inline(always)]
    #[must_use]
    pub fn cpdma_fhost_channel(
        &mut self,
    ) -> CpdmaFhostChannelW<CpswCpdmaIntCpdmaFhIntmaskSetRegSpec> {
        CpdmaFhostChannelW::new(self, 6)
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
