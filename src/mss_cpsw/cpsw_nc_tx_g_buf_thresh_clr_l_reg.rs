#[doc = "Register `CPSW_NC_TX_G_BUF_THRESH_CLR_L_REG` reader"]
pub type R = crate::R<CpswNcTxGBufThreshClrLRegSpec>;
#[doc = "Register `CPSW_NC_TX_G_BUF_THRESH_CLR_L_REG` writer"]
pub type W = crate::W<CpswNcTxGBufThreshClrLRegSpec>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGBufThreshClrLRegSpec> {
        PriorityBasedFlowW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGBufThreshClrLRegSpec> {
        PriorityBasedFlowW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGBufThreshClrLRegSpec> {
        PriorityBasedFlowW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGBufThreshClrLRegSpec> {
        PriorityBasedFlowW::new(self, 24)
    }
}
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcTxGBufThreshClrLRegSpec;
impl crate::RegisterSpec for CpswNcTxGBufThreshClrLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcTxGBufThreshClrLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_tx_g_buf_thresh_clr_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcTxGBufThreshClrLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_TX_G_BUF_THRESH_CLR_L_REG to value 0"]
impl crate::Resettable for CpswNcTxGBufThreshClrLRegSpec {
    const RESET_VALUE: u32 = 0;
}
