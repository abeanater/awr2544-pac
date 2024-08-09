#[doc = "Register `CSI2_CTRL2` reader"]
pub type R = crate::R<Csi2Ctrl2Spec>;
#[doc = "Register `CSI2_CTRL2` writer"]
pub type W = crate::W<Csi2Ctrl2Spec>;
#[doc = "Field `RESERVED2` reader - "]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpClkRatio {
    #[doc = "0: The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    Ratio2 = 0,
    #[doc = "1: The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    Ratio3andhigher = 1,
}
impl From<VpClkRatio> for bool {
    #[inline(always)]
    fn from(variant: VpClkRatio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_CLK_RATIO` reader - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
pub type VpClkRatioR = crate::BitReader<VpClkRatio>;
impl VpClkRatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpClkRatio {
        match self.bits {
            false => VpClkRatio::Ratio2,
            true => VpClkRatio::Ratio3andhigher,
        }
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    #[inline(always)]
    pub fn is_ratio2(&self) -> bool {
        *self == VpClkRatio::Ratio2
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    #[inline(always)]
    pub fn is_ratio3andhigher(&self) -> bool {
        *self == VpClkRatio::Ratio3andhigher
    }
}
#[doc = "Field `VP_CLK_RATIO` writer - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
pub type VpClkRatioW<'a, REG> = crate::BitWriter<'a, REG, VpClkRatio>;
impl<'a, REG> VpClkRatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 2. The duty cycle of VP.PCLK is 50/50."]
    #[inline(always)]
    pub fn ratio2(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkRatio::Ratio2)
    }
    #[doc = "The clock VP.PCLK is the clock VP.CLK divided by 3 or more. The duty cycle of VP.PCLK is not 50/50 for odd ratio numbers (3,5,7,...)."]
    #[inline(always)]
    pub fn ratio3andhigher(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkRatio::Ratio3andhigher)
    }
}
#[doc = "Field `RESERVED3` reader - "]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - "]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:6\\]
Defines the size of the video port data bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VpDataBusWidth {
    #[doc = "0: 16-bits data width (LSB of the 24-bit video port data bus)"]
    F16 = 0,
    #[doc = "1: 18-bits data width (LSB of the 24-bit video port data bus)"]
    F18 = 1,
    #[doc = "2: 24-bits data width (LSB of the 24-bit video port data bus)"]
    F24 = 2,
}
impl From<VpDataBusWidth> for u8 {
    #[inline(always)]
    fn from(variant: VpDataBusWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VpDataBusWidth {
    type Ux = u8;
}
impl crate::IsEnum for VpDataBusWidth {}
#[doc = "Field `VP_DATA_BUS_WIDTH` reader - 7:6\\]
Defines the size of the video port data bus"]
pub type VpDataBusWidthR = crate::FieldReader<VpDataBusWidth>;
impl VpDataBusWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VpDataBusWidth> {
        match self.bits {
            0 => Some(VpDataBusWidth::F16),
            1 => Some(VpDataBusWidth::F18),
            2 => Some(VpDataBusWidth::F24),
            _ => None,
        }
    }
    #[doc = "16-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f16(&self) -> bool {
        *self == VpDataBusWidth::F16
    }
    #[doc = "18-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f18(&self) -> bool {
        *self == VpDataBusWidth::F18
    }
    #[doc = "24-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn is_f24(&self) -> bool {
        *self == VpDataBusWidth::F24
    }
}
#[doc = "Field `VP_DATA_BUS_WIDTH` writer - 7:6\\]
Defines the size of the video port data bus"]
pub type VpDataBusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, VpDataBusWidth>;
impl<'a, REG> VpDataBusWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f16(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F16)
    }
    #[doc = "18-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f18(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F18)
    }
    #[doc = "24-bits data width (LSB of the 24-bit video port data bus)"]
    #[inline(always)]
    pub fn f24(self) -> &'a mut crate::W<REG> {
        self.variant(VpDataBusWidth::F24)
    }
}
#[doc = "8:8\\]
VP clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpClkPol {
    #[doc = "0: The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    Falling = 0,
    #[doc = "1: The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    Rising = 1,
}
impl From<VpClkPol> for bool {
    #[inline(always)]
    fn from(variant: VpClkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_CLK_POL` reader - 8:8\\]
VP clock polarity"]
pub type VpClkPolR = crate::BitReader<VpClkPol>;
impl VpClkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpClkPol {
        match self.bits {
            false => VpClkPol::Falling,
            true => VpClkPol::Rising,
        }
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == VpClkPol::Falling
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == VpClkPol::Rising
    }
}
#[doc = "Field `VP_CLK_POL` writer - 8:8\\]
VP clock polarity"]
pub type VpClkPolW<'a, REG> = crate::BitWriter<'a, REG, VpClkPol>;
impl<'a, REG> VpClkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock falling edge. The module connected to the VP shall drive the data on the pixel clock rising edge."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkPol::Falling)
    }
    #[doc = "The CSI2 Protocol Engine module captures the data on the VP on the pixel clock raising edge. The module connected to the VP shall drive the data on the pixel clock falling edge."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(VpClkPol::Rising)
    }
}
#[doc = "9:9\\]
VP data enable signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpDePol {
    #[doc = "0: DE signal on the video port is active low."]
    Low = 0,
    #[doc = "1: DE signal on the video port is active high."]
    High = 1,
}
impl From<VpDePol> for bool {
    #[inline(always)]
    fn from(variant: VpDePol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_DE_POL` reader - 9:9\\]
VP data enable signal polarity"]
pub type VpDePolR = crate::BitReader<VpDePol>;
impl VpDePolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpDePol {
        match self.bits {
            false => VpDePol::Low,
            true => VpDePol::High,
        }
    }
    #[doc = "DE signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpDePol::Low
    }
    #[doc = "DE signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpDePol::High
    }
}
#[doc = "Field `VP_DE_POL` writer - 9:9\\]
VP data enable signal polarity"]
pub type VpDePolW<'a, REG> = crate::BitWriter<'a, REG, VpDePol>;
impl<'a, REG> VpDePolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpDePol::Low)
    }
    #[doc = "DE signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpDePol::High)
    }
}
#[doc = "10:10\\]
VP horizontal synchronization signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpHsyncPol {
    #[doc = "0: HSYNC signal on the video port is active low."]
    Low = 0,
    #[doc = "1: HSYNC signal on the video port is active high."]
    High = 1,
}
impl From<VpHsyncPol> for bool {
    #[inline(always)]
    fn from(variant: VpHsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_HSYNC_POL` reader - 10:10\\]
VP horizontal synchronization signal polarity"]
pub type VpHsyncPolR = crate::BitReader<VpHsyncPol>;
impl VpHsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpHsyncPol {
        match self.bits {
            false => VpHsyncPol::Low,
            true => VpHsyncPol::High,
        }
    }
    #[doc = "HSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpHsyncPol::Low
    }
    #[doc = "HSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpHsyncPol::High
    }
}
#[doc = "Field `VP_HSYNC_POL` writer - 10:10\\]
VP horizontal synchronization signal polarity"]
pub type VpHsyncPolW<'a, REG> = crate::BitWriter<'a, REG, VpHsyncPol>;
impl<'a, REG> VpHsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncPol::Low)
    }
    #[doc = "HSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpHsyncPol::High)
    }
}
#[doc = "11:11\\]
VP vertical synchronization signal polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VpVsyncPol {
    #[doc = "0: VSYNC signal on the video port is active low."]
    Low = 0,
    #[doc = "1: VSYNC signal on the video port is active high."]
    High = 1,
}
impl From<VpVsyncPol> for bool {
    #[inline(always)]
    fn from(variant: VpVsyncPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_VSYNC_POL` reader - 11:11\\]
VP vertical synchronization signal polarity"]
pub type VpVsyncPolR = crate::BitReader<VpVsyncPol>;
impl VpVsyncPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VpVsyncPol {
        match self.bits {
            false => VpVsyncPol::Low,
            true => VpVsyncPol::High,
        }
    }
    #[doc = "VSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VpVsyncPol::Low
    }
    #[doc = "VSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VpVsyncPol::High
    }
}
#[doc = "Field `VP_VSYNC_POL` writer - 11:11\\]
VP vertical synchronization signal polarity"]
pub type VpVsyncPolW<'a, REG> = crate::BitWriter<'a, REG, VpVsyncPol>;
impl<'a, REG> VpVsyncPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VSYNC signal on the video port is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncPol::Low)
    }
    #[doc = "VSYNC signal on the video port is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(VpVsyncPol::High)
    }
}
#[doc = "13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP2_NB_LINE_BUFFER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LineBuffer {
    #[doc = "0: No line buffer"]
    F0 = 0,
    #[doc = "1: 1 line buffer"]
    F1 = 1,
    #[doc = "2: 2 line buffers"]
    F2 = 2,
}
impl From<LineBuffer> for u8 {
    #[inline(always)]
    fn from(variant: LineBuffer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LineBuffer {
    type Ux = u8;
}
impl crate::IsEnum for LineBuffer {}
#[doc = "Field `LINE_BUFFER` reader - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP2_NB_LINE_BUFFER."]
pub type LineBufferR = crate::FieldReader<LineBuffer>;
impl LineBufferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LineBuffer> {
        match self.bits {
            0 => Some(LineBuffer::F0),
            1 => Some(LineBuffer::F1),
            2 => Some(LineBuffer::F2),
            _ => None,
        }
    }
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == LineBuffer::F0
    }
    #[doc = "1 line buffer"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == LineBuffer::F1
    }
    #[doc = "2 line buffers"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == LineBuffer::F2
    }
}
#[doc = "Field `LINE_BUFFER` writer - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP2_NB_LINE_BUFFER."]
pub type LineBufferW<'a, REG> = crate::FieldWriter<'a, REG, 2, LineBuffer>;
impl<'a, REG> LineBufferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F0)
    }
    #[doc = "1 line buffer"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F1)
    }
    #[doc = "2 line buffers"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(LineBuffer::F2)
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
    #[inline(always)]
    pub fn vp_clk_ratio(&self) -> VpClkRatioR {
        VpClkRatioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Defines the size of the video port data bus"]
    #[inline(always)]
    pub fn vp_data_bus_width(&self) -> VpDataBusWidthR {
        VpDataBusWidthR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
VP clock polarity"]
    #[inline(always)]
    pub fn vp_clk_pol(&self) -> VpClkPolR {
        VpClkPolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
VP data enable signal polarity"]
    #[inline(always)]
    pub fn vp_de_pol(&self) -> VpDePolR {
        VpDePolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
VP horizontal synchronization signal polarity"]
    #[inline(always)]
    pub fn vp_hsync_pol(&self) -> VpHsyncPolR {
        VpHsyncPolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
VP vertical synchronization signal polarity"]
    #[inline(always)]
    pub fn vp_vsync_pol(&self) -> VpVsyncPolR {
        VpVsyncPolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP2_NB_LINE_BUFFER."]
    #[inline(always)]
    pub fn line_buffer(&self) -> LineBufferR {
        LineBufferR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2Ctrl2Spec> {
        Reserved2W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
The field indicates the clock ratio between VP.CLK and VP.PCLK. The clock VP.PCLK is generated from VP.CLK. It is divided down. The information is only used when the video port is used to provide data in command mode. In the case of video mode, it is not used."]
    #[inline(always)]
    #[must_use]
    pub fn vp_clk_ratio(&mut self) -> VpClkRatioW<Csi2Ctrl2Spec> {
        VpClkRatioW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Csi2Ctrl2Spec> {
        Reserved3W::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Defines the size of the video port data bus"]
    #[inline(always)]
    #[must_use]
    pub fn vp_data_bus_width(&mut self) -> VpDataBusWidthW<Csi2Ctrl2Spec> {
        VpDataBusWidthW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
VP clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_clk_pol(&mut self) -> VpClkPolW<Csi2Ctrl2Spec> {
        VpClkPolW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
VP data enable signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_de_pol(&mut self) -> VpDePolW<Csi2Ctrl2Spec> {
        VpDePolW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
VP horizontal synchronization signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_hsync_pol(&mut self) -> VpHsyncPolW<Csi2Ctrl2Spec> {
        VpHsyncPolW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
VP vertical synchronization signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vp_vsync_pol(&mut self) -> VpVsyncPolW<Csi2Ctrl2Spec> {
        VpVsyncPolW::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Number of line buffers to be used while receiving data on the video port. The valid values are from 0 to CSI2_GNQ.VP2_NB_LINE_BUFFER."]
    #[inline(always)]
    #[must_use]
    pub fn line_buffer(&mut self) -> LineBufferW<Csi2Ctrl2Spec> {
        LineBufferW::new(self, 12)
    }
}
#[doc = "Additional control bits for use with Video Port 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2Ctrl2Spec;
impl crate::RegisterSpec for Csi2Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_ctrl2::R`](R) reader structure"]
impl crate::Readable for Csi2Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_ctrl2::W`](W) writer structure"]
impl crate::Writable for Csi2Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_CTRL2 to value 0"]
impl crate::Resettable for Csi2Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
