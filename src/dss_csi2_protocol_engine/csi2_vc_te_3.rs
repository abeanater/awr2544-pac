#[doc = "Register `CSI2_VC_TE_3` reader"]
pub type R = crate::R<Csi2VcTe3Spec>;
#[doc = "Register `CSI2_VC_TE_3` writer"]
pub type W = crate::W<Csi2VcTe3Spec>;
#[doc = "Field `TE_SIZE` reader - 23:0\\]
Defines the number of byte (payload data excluding the check -sum) to be sent. The write into the register CSI2_VC_LONG_PACKET_HEADER shall be performed by the user before sending data from the register CSI2_VC_LONG_PACKET_PAYLOAD. The register value is decremented for every byte sent of the CSI2 link. At the end of the transfer (TE_SIZE=0), the bit-field TE_EN is reset by HW. The DMA_request shall be asserted when the trigger is received in order to receive data in the TX FIFO. It shall not be used until all data (TE_SIZE) have been received in the FIFO."]
pub type TeSizeR = crate::FieldReader<u32>;
#[doc = "Field `TE_SIZE` writer - 23:0\\]
Defines the number of byte (payload data excluding the check -sum) to be sent. The write into the register CSI2_VC_LONG_PACKET_HEADER shall be performed by the user before sending data from the register CSI2_VC_LONG_PACKET_PAYLOAD. The register value is decremented for every byte sent of the CSI2 link. At the end of the transfer (TE_SIZE=0), the bit-field TE_EN is reset by HW. The DMA_request shall be asserted when the trigger is received in order to receive data in the TX FIFO. It shall not be used until all data (TE_SIZE) have been received in the FIFO."]
pub type TeSizeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "28:28\\]
Selection between TE0 and TE1 cmos signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeLineNb {
    #[doc = "0: TE0 cmos input line is selected"]
    Te0 = 0,
    #[doc = "1: TE1 cmos input line is selected"]
    Te1 = 1,
}
impl From<TeLineNb> for bool {
    #[inline(always)]
    fn from(variant: TeLineNb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_LINE_NB` reader - 28:28\\]
Selection between TE0 and TE1 cmos signals."]
pub type TeLineNbR = crate::BitReader<TeLineNb>;
impl TeLineNbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeLineNb {
        match self.bits {
            false => TeLineNb::Te0,
            true => TeLineNb::Te1,
        }
    }
    #[doc = "TE0 cmos input line is selected"]
    #[inline(always)]
    pub fn is_te0(&self) -> bool {
        *self == TeLineNb::Te0
    }
    #[doc = "TE1 cmos input line is selected"]
    #[inline(always)]
    pub fn is_te1(&self) -> bool {
        *self == TeLineNb::Te1
    }
}
#[doc = "Field `TE_LINE_NB` writer - 28:28\\]
Selection between TE0 and TE1 cmos signals."]
pub type TeLineNbW<'a, REG> = crate::BitWriter<'a, REG, TeLineNb>;
impl<'a, REG> TeLineNbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TE0 cmos input line is selected"]
    #[inline(always)]
    pub fn te0(self) -> &'a mut crate::W<REG> {
        self.variant(TeLineNb::Te0)
    }
    #[doc = "TE1 cmos input line is selected"]
    #[inline(always)]
    pub fn te1(self) -> &'a mut crate::W<REG> {
        self.variant(TeLineNb::Te1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeLine {
    #[doc = "0: Disabled the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is used for the automactic data transfer."]
    Disable = 0,
    #[doc = "1: Enables the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is not used for the automactic data transfer."]
    Enable = 1,
}
impl From<TeLine> for bool {
    #[inline(always)]
    fn from(variant: TeLine) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_LINE` reader - "]
pub type TeLineR = crate::BitReader<TeLine>;
impl TeLineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeLine {
        match self.bits {
            false => TeLine::Disable,
            true => TeLine::Enable,
        }
    }
    #[doc = "Disabled the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is used for the automactic data transfer."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TeLine::Disable
    }
    #[doc = "Enables the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is not used for the automactic data transfer."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TeLine::Enable
    }
}
#[doc = "Field `TE_LINE` writer - "]
pub type TeLineW<'a, REG> = crate::BitWriter<'a, REG, TeLine>;
impl<'a, REG> TeLineW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is used for the automactic data transfer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TeLine::Disable)
    }
    #[doc = "Enables the TE CMOS signalling for the automatic data transfer. The CSI2 PHY trigger is not used for the automactic data transfer."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TeLine::Enable)
    }
}
#[doc = "30:30\\]
Tearing Effect Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeEn {
    #[doc = "0: Disables the automatic transfer. The user shall use the interruption in order to know when TE PHY trigger is received or when the TE is detected on the input CMOS signals. The HW reset the bit-field when the transfer is completed (TE_SIZE=0)."]
    Disable = 0,
    #[doc = "1: Enables the automatic transfer of the data using the TE PHY trigger or one of the TE input signals as a synchronization event. The bit-field TE_LINE defines if the CMOS signal is used or if the PHY trigger is used."]
    Enable = 1,
}
impl From<TeEn> for bool {
    #[inline(always)]
    fn from(variant: TeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_EN` reader - 30:30\\]
Tearing Effect Control"]
pub type TeEnR = crate::BitReader<TeEn>;
impl TeEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeEn {
        match self.bits {
            false => TeEn::Disable,
            true => TeEn::Enable,
        }
    }
    #[doc = "Disables the automatic transfer. The user shall use the interruption in order to know when TE PHY trigger is received or when the TE is detected on the input CMOS signals. The HW reset the bit-field when the transfer is completed (TE_SIZE=0)."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TeEn::Disable
    }
    #[doc = "Enables the automatic transfer of the data using the TE PHY trigger or one of the TE input signals as a synchronization event. The bit-field TE_LINE defines if the CMOS signal is used or if the PHY trigger is used."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TeEn::Enable
    }
}
#[doc = "Field `TE_EN` writer - 30:30\\]
Tearing Effect Control"]
pub type TeEnW<'a, REG> = crate::BitWriter<'a, REG, TeEn>;
impl<'a, REG> TeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the automatic transfer. The user shall use the interruption in order to know when TE PHY trigger is received or when the TE is detected on the input CMOS signals. The HW reset the bit-field when the transfer is completed (TE_SIZE=0)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TeEn::Disable)
    }
    #[doc = "Enables the automatic transfer of the data using the TE PHY trigger or one of the TE input signals as a synchronization event. The bit-field TE_LINE defines if the CMOS signal is used or if the PHY trigger is used."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TeEn::Enable)
    }
}
#[doc = "31:31\\]
Manual control of the start of the transfer. The user can use the TE interrupt in order to know that the TE trigger has been received prior to set the TE_START bit-field. It is not mandatory to use the TE interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeStart {
    #[doc = "0: Indicates the end of the transfer. The bit can be used by user to cancel the transfer if not already started. The FIFO shall be flushed by SW to ensure there is no data remaining in it."]
    Disable = 0,
    #[doc = "1: Starts the transfer of the data. The size is defined in TE_SIZE. The bit-field is set until the transfer is completed. It is reset by HW when the transfer is completed."]
    Enable = 1,
}
impl From<TeStart> for bool {
    #[inline(always)]
    fn from(variant: TeStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE_START` reader - 31:31\\]
Manual control of the start of the transfer. The user can use the TE interrupt in order to know that the TE trigger has been received prior to set the TE_START bit-field. It is not mandatory to use the TE interrupt."]
pub type TeStartR = crate::BitReader<TeStart>;
impl TeStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TeStart {
        match self.bits {
            false => TeStart::Disable,
            true => TeStart::Enable,
        }
    }
    #[doc = "Indicates the end of the transfer. The bit can be used by user to cancel the transfer if not already started. The FIFO shall be flushed by SW to ensure there is no data remaining in it."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TeStart::Disable
    }
    #[doc = "Starts the transfer of the data. The size is defined in TE_SIZE. The bit-field is set until the transfer is completed. It is reset by HW when the transfer is completed."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TeStart::Enable
    }
}
#[doc = "Field `TE_START` writer - 31:31\\]
Manual control of the start of the transfer. The user can use the TE interrupt in order to know that the TE trigger has been received prior to set the TE_START bit-field. It is not mandatory to use the TE interrupt."]
pub type TeStartW<'a, REG> = crate::BitWriter<'a, REG, TeStart>;
impl<'a, REG> TeStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates the end of the transfer. The bit can be used by user to cancel the transfer if not already started. The FIFO shall be flushed by SW to ensure there is no data remaining in it."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TeStart::Disable)
    }
    #[doc = "Starts the transfer of the data. The size is defined in TE_SIZE. The bit-field is set until the transfer is completed. It is reset by HW when the transfer is completed."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TeStart::Enable)
    }
}
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the number of byte (payload data excluding the check -sum) to be sent. The write into the register CSI2_VC_LONG_PACKET_HEADER shall be performed by the user before sending data from the register CSI2_VC_LONG_PACKET_PAYLOAD. The register value is decremented for every byte sent of the CSI2 link. At the end of the transfer (TE_SIZE=0), the bit-field TE_EN is reset by HW. The DMA_request shall be asserted when the trigger is received in order to receive data in the TX FIFO. It shall not be used until all data (TE_SIZE) have been received in the FIFO."]
    #[inline(always)]
    pub fn te_size(&self) -> TeSizeR {
        TeSizeR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 28 - 28:28\\]
Selection between TE0 and TE1 cmos signals."]
    #[inline(always)]
    pub fn te_line_nb(&self) -> TeLineNbR {
        TeLineNbR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn te_line(&self) -> TeLineR {
        TeLineR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Tearing Effect Control"]
    #[inline(always)]
    pub fn te_en(&self) -> TeEnR {
        TeEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Manual control of the start of the transfer. The user can use the TE interrupt in order to know that the TE trigger has been received prior to set the TE_START bit-field. It is not mandatory to use the TE interrupt."]
    #[inline(always)]
    pub fn te_start(&self) -> TeStartR {
        TeStartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the number of byte (payload data excluding the check -sum) to be sent. The write into the register CSI2_VC_LONG_PACKET_HEADER shall be performed by the user before sending data from the register CSI2_VC_LONG_PACKET_PAYLOAD. The register value is decremented for every byte sent of the CSI2 link. At the end of the transfer (TE_SIZE=0), the bit-field TE_EN is reset by HW. The DMA_request shall be asserted when the trigger is received in order to receive data in the TX FIFO. It shall not be used until all data (TE_SIZE) have been received in the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn te_size(&mut self) -> TeSizeW<Csi2VcTe3Spec> {
        TeSizeW::new(self, 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Selection between TE0 and TE1 cmos signals."]
    #[inline(always)]
    #[must_use]
    pub fn te_line_nb(&mut self) -> TeLineNbW<Csi2VcTe3Spec> {
        TeLineNbW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn te_line(&mut self) -> TeLineW<Csi2VcTe3Spec> {
        TeLineW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Tearing Effect Control"]
    #[inline(always)]
    #[must_use]
    pub fn te_en(&mut self) -> TeEnW<Csi2VcTe3Spec> {
        TeEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Manual control of the start of the transfer. The user can use the TE interrupt in order to know that the TE trigger has been received prior to set the TE_START bit-field. It is not mandatory to use the TE interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn te_start(&mut self) -> TeStartW<Csi2VcTe3Spec> {
        TeStartW::new(self, 31)
    }
}
#[doc = "CONTROL REGISTER - Virtual channel This register controls the tearing effect logic. It defines the size of the transfer when TE occurs and enables the automatic TE mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_te_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_te_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VcTe3Spec;
impl crate::RegisterSpec for Csi2VcTe3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vc_te_3::R`](R) reader structure"]
impl crate::Readable for Csi2VcTe3Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vc_te_3::W`](W) writer structure"]
impl crate::Writable for Csi2VcTe3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VC_TE_3 to value 0"]
impl crate::Resettable for Csi2VcTe3Spec {
    const RESET_VALUE: u32 = 0;
}
