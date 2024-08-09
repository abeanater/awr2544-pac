#[doc = "Register `CPSW_NC_TX_G_OFLOW_THRESH_CLR_REG` reader"]
pub type R = crate::R<CpswNcTxGOflowThreshClrRegSpec>;
#[doc = "Register `CPSW_NC_TX_G_OFLOW_THRESH_CLR_REG` writer"]
pub type W = crate::W<CpswNcTxGOflowThreshClrRegSpec>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_BASED_FLOW` reader - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
pub type PriorityBasedFlowR = crate::FieldReader;
#[doc = "Field `PRIORITY_BASED_FLOW` writer - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
pub type PriorityBasedFlowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
    #[inline(always)]
    pub fn priority_based_flow(&self) -> PriorityBasedFlowR {
        PriorityBasedFlowR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
    #[inline(always)]
    #[must_use]
    pub fn priority_based_flow(&mut self) -> PriorityBasedFlowW<CpswNcTxGOflowThreshClrRegSpec> {
        PriorityBasedFlowW::new(self, 28)
    }
}
#[doc = "CPSW PFC Tx Global Out Flow Threshold Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_tx_g_oflow_thresh_clr_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_tx_g_oflow_thresh_clr_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcTxGOflowThreshClrRegSpec;
impl crate::RegisterSpec for CpswNcTxGOflowThreshClrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_tx_g_oflow_thresh_clr_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcTxGOflowThreshClrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_tx_g_oflow_thresh_clr_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcTxGOflowThreshClrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_TX_G_OFLOW_THRESH_CLR_REG to value 0"]
impl crate::Resettable for CpswNcTxGOflowThreshClrRegSpec {
    const RESET_VALUE: u32 = 0;
}
