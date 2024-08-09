#[doc = "Register `CSI2_GNQ` reader"]
pub type R = crate::R<Csi2GnqSpec>;
#[doc = "Register `CSI2_GNQ` writer"]
pub type W = crate::W<Csi2GnqSpec>;
#[doc = "2:0\\]
Determines the data TX FIFO depth (33-bit words) on the slave port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxFifodepth {
    #[doc = "4: 32x 33 bits"]
    F32 = 4,
    #[doc = "5: 64x 33 bits"]
    F64 = 5,
    #[doc = "6: 128 x 33 bits"]
    F128 = 6,
    #[doc = "7: 256 x 33 bits"]
    F256 = 7,
}
impl From<TxFifodepth> for u8 {
    #[inline(always)]
    fn from(variant: TxFifodepth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxFifodepth {
    type Ux = u8;
}
impl crate::IsEnum for TxFifodepth {}
#[doc = "Field `TX_FIFODEPTH` reader - 2:0\\]
Determines the data TX FIFO depth (33-bit words) on the slave port."]
pub type TxFifodepthR = crate::FieldReader<TxFifodepth>;
impl TxFifodepthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxFifodepth> {
        match self.bits {
            4 => Some(TxFifodepth::F32),
            5 => Some(TxFifodepth::F64),
            6 => Some(TxFifodepth::F128),
            7 => Some(TxFifodepth::F256),
            _ => None,
        }
    }
    #[doc = "32x 33 bits"]
    #[inline(always)]
    pub fn is_f32(&self) -> bool {
        *self == TxFifodepth::F32
    }
    #[doc = "64x 33 bits"]
    #[inline(always)]
    pub fn is_f64(&self) -> bool {
        *self == TxFifodepth::F64
    }
    #[doc = "128 x 33 bits"]
    #[inline(always)]
    pub fn is_f128(&self) -> bool {
        *self == TxFifodepth::F128
    }
    #[doc = "256 x 33 bits"]
    #[inline(always)]
    pub fn is_f256(&self) -> bool {
        *self == TxFifodepth::F256
    }
}
#[doc = "Field `TX_FIFODEPTH` writer - 2:0\\]
Determines the data TX FIFO depth (33-bit words) on the slave port."]
pub type TxFifodepthW<'a, REG> = crate::FieldWriter<'a, REG, 3, TxFifodepth>;
impl<'a, REG> TxFifodepthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32x 33 bits"]
    #[inline(always)]
    pub fn f32(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifodepth::F32)
    }
    #[doc = "64x 33 bits"]
    #[inline(always)]
    pub fn f64(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifodepth::F64)
    }
    #[doc = "128 x 33 bits"]
    #[inline(always)]
    pub fn f128(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifodepth::F128)
    }
    #[doc = "256 x 33 bits"]
    #[inline(always)]
    pub fn f256(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifodepth::F256)
    }
}
#[doc = "5:3\\]
Determines the data RX FIFO depth (32-bit words) on the slave port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxFifodepth {
    #[doc = "4: 32x 33 bits"]
    F32 = 4,
    #[doc = "5: 64x 33 bits"]
    F64 = 5,
    #[doc = "6: 128 x 33 bits"]
    F128 = 6,
    #[doc = "7: 256 x 33 bits"]
    F256 = 7,
}
impl From<RxFifodepth> for u8 {
    #[inline(always)]
    fn from(variant: RxFifodepth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxFifodepth {
    type Ux = u8;
}
impl crate::IsEnum for RxFifodepth {}
#[doc = "Field `RX_FIFODEPTH` reader - 5:3\\]
Determines the data RX FIFO depth (32-bit words) on the slave port."]
pub type RxFifodepthR = crate::FieldReader<RxFifodepth>;
impl RxFifodepthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxFifodepth> {
        match self.bits {
            4 => Some(RxFifodepth::F32),
            5 => Some(RxFifodepth::F64),
            6 => Some(RxFifodepth::F128),
            7 => Some(RxFifodepth::F256),
            _ => None,
        }
    }
    #[doc = "32x 33 bits"]
    #[inline(always)]
    pub fn is_f32(&self) -> bool {
        *self == RxFifodepth::F32
    }
    #[doc = "64x 33 bits"]
    #[inline(always)]
    pub fn is_f64(&self) -> bool {
        *self == RxFifodepth::F64
    }
    #[doc = "128 x 33 bits"]
    #[inline(always)]
    pub fn is_f128(&self) -> bool {
        *self == RxFifodepth::F128
    }
    #[doc = "256 x 33 bits"]
    #[inline(always)]
    pub fn is_f256(&self) -> bool {
        *self == RxFifodepth::F256
    }
}
#[doc = "Field `RX_FIFODEPTH` writer - 5:3\\]
Determines the data RX FIFO depth (32-bit words) on the slave port."]
pub type RxFifodepthW<'a, REG> = crate::FieldWriter<'a, REG, 3, RxFifodepth>;
impl<'a, REG> RxFifodepthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32x 33 bits"]
    #[inline(always)]
    pub fn f32(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifodepth::F32)
    }
    #[doc = "64x 33 bits"]
    #[inline(always)]
    pub fn f64(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifodepth::F64)
    }
    #[doc = "128 x 33 bits"]
    #[inline(always)]
    pub fn f128(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifodepth::F128)
    }
    #[doc = "256 x 33 bits"]
    #[inline(always)]
    pub fn f256(self) -> &'a mut crate::W<REG> {
        self.variant(RxFifodepth::F256)
    }
}
#[doc = "8:6\\]
Determines the number of DMA_REQ signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NbDmaRequest {
    #[doc = "0: No DMA request"]
    F0 = 0,
    #[doc = "1: 1 DMA request"]
    F1 = 1,
    #[doc = "2: 2 DMA requests"]
    F2 = 2,
    #[doc = "3: 3 DMA requests"]
    F3 = 3,
    #[doc = "4: 4 DMA requests"]
    F4 = 4,
}
impl From<NbDmaRequest> for u8 {
    #[inline(always)]
    fn from(variant: NbDmaRequest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NbDmaRequest {
    type Ux = u8;
}
impl crate::IsEnum for NbDmaRequest {}
#[doc = "Field `NB_DMA_REQUEST` reader - 8:6\\]
Determines the number of DMA_REQ signals."]
pub type NbDmaRequestR = crate::FieldReader<NbDmaRequest>;
impl NbDmaRequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NbDmaRequest> {
        match self.bits {
            0 => Some(NbDmaRequest::F0),
            1 => Some(NbDmaRequest::F1),
            2 => Some(NbDmaRequest::F2),
            3 => Some(NbDmaRequest::F3),
            4 => Some(NbDmaRequest::F4),
            _ => None,
        }
    }
    #[doc = "No DMA request"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == NbDmaRequest::F0
    }
    #[doc = "1 DMA request"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == NbDmaRequest::F1
    }
    #[doc = "2 DMA requests"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == NbDmaRequest::F2
    }
    #[doc = "3 DMA requests"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == NbDmaRequest::F3
    }
    #[doc = "4 DMA requests"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == NbDmaRequest::F4
    }
}
#[doc = "Field `NB_DMA_REQUEST` writer - 8:6\\]
Determines the number of DMA_REQ signals."]
pub type NbDmaRequestW<'a, REG> = crate::FieldWriter<'a, REG, 3, NbDmaRequest>;
impl<'a, REG> NbDmaRequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA request"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(NbDmaRequest::F0)
    }
    #[doc = "1 DMA request"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(NbDmaRequest::F1)
    }
    #[doc = "2 DMA requests"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(NbDmaRequest::F2)
    }
    #[doc = "3 DMA requests"]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(NbDmaRequest::F3)
    }
    #[doc = "4 DMA requests"]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(NbDmaRequest::F4)
    }
}
#[doc = "11:9\\]
Determines the number of data lanes supported by the CSI2 protocol engine .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NbDataLanes {
    #[doc = "1: 1 Data lane"]
    F1 = 1,
    #[doc = "2: 2 Data lanes"]
    F2 = 2,
    #[doc = "3: 3 Data lanes"]
    F3 = 3,
    #[doc = "4: 4 Data lanes"]
    F4 = 4,
}
impl From<NbDataLanes> for u8 {
    #[inline(always)]
    fn from(variant: NbDataLanes) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NbDataLanes {
    type Ux = u8;
}
impl crate::IsEnum for NbDataLanes {}
#[doc = "Field `NB_DATA_LANES` reader - 11:9\\]
Determines the number of data lanes supported by the CSI2 protocol engine ."]
pub type NbDataLanesR = crate::FieldReader<NbDataLanes>;
impl NbDataLanesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NbDataLanes> {
        match self.bits {
            1 => Some(NbDataLanes::F1),
            2 => Some(NbDataLanes::F2),
            3 => Some(NbDataLanes::F3),
            4 => Some(NbDataLanes::F4),
            _ => None,
        }
    }
    #[doc = "1 Data lane"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == NbDataLanes::F1
    }
    #[doc = "2 Data lanes"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == NbDataLanes::F2
    }
    #[doc = "3 Data lanes"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == NbDataLanes::F3
    }
    #[doc = "4 Data lanes"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == NbDataLanes::F4
    }
}
#[doc = "Field `NB_DATA_LANES` writer - 11:9\\]
Determines the number of data lanes supported by the CSI2 protocol engine ."]
pub type NbDataLanesW<'a, REG> = crate::FieldWriter<'a, REG, 3, NbDataLanes>;
impl<'a, REG> NbDataLanesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Data lane"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(NbDataLanes::F1)
    }
    #[doc = "2 Data lanes"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(NbDataLanes::F2)
    }
    #[doc = "3 Data lanes"]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(NbDataLanes::F3)
    }
    #[doc = "4 Data lanes"]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(NbDataLanes::F4)
    }
}
#[doc = "14:12\\]
Determines the video line buffer size associated to video port #1 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vp1LineBufferSize {
    #[doc = "1: 512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    F1 = 1,
    #[doc = "2: 682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    F2 = 2,
    #[doc = "3: 853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    F3 = 3,
    #[doc = "4: 1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    F4 = 4,
    #[doc = "5: 1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    F5 = 5,
    #[doc = "6: 1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    F6 = 6,
}
impl From<Vp1LineBufferSize> for u8 {
    #[inline(always)]
    fn from(variant: Vp1LineBufferSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vp1LineBufferSize {
    type Ux = u8;
}
impl crate::IsEnum for Vp1LineBufferSize {}
#[doc = "Field `VP1_LINE_BUFFER_SIZE` reader - 14:12\\]
Determines the video line buffer size associated to video port #1 ."]
pub type Vp1LineBufferSizeR = crate::FieldReader<Vp1LineBufferSize>;
impl Vp1LineBufferSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vp1LineBufferSize> {
        match self.bits {
            1 => Some(Vp1LineBufferSize::F1),
            2 => Some(Vp1LineBufferSize::F2),
            3 => Some(Vp1LineBufferSize::F3),
            4 => Some(Vp1LineBufferSize::F4),
            5 => Some(Vp1LineBufferSize::F5),
            6 => Some(Vp1LineBufferSize::F6),
            _ => None,
        }
    }
    #[doc = "512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Vp1LineBufferSize::F1
    }
    #[doc = "682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Vp1LineBufferSize::F2
    }
    #[doc = "853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == Vp1LineBufferSize::F3
    }
    #[doc = "1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == Vp1LineBufferSize::F4
    }
    #[doc = "1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == Vp1LineBufferSize::F5
    }
    #[doc = "1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == Vp1LineBufferSize::F6
    }
}
#[doc = "Field `VP1_LINE_BUFFER_SIZE` writer - 14:12\\]
Determines the video line buffer size associated to video port #1 ."]
pub type Vp1LineBufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vp1LineBufferSize>;
impl<'a, REG> Vp1LineBufferSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F1)
    }
    #[doc = "682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F2)
    }
    #[doc = "853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F3)
    }
    #[doc = "1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F4)
    }
    #[doc = "1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    #[inline(always)]
    pub fn f5(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F5)
    }
    #[doc = "1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    #[inline(always)]
    pub fn f6(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1LineBufferSize::F6)
    }
}
#[doc = "Field `RESERVED1` reader - 15:15\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 15:15\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "17:16\\]
Determines the number of video buffer lines associated to video port #1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vp1NbLineBuffer {
    #[doc = "0: No line buffer"]
    F0 = 0,
    #[doc = "1: 1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    F1 = 1,
    #[doc = "2: 2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    F2 = 2,
}
impl From<Vp1NbLineBuffer> for u8 {
    #[inline(always)]
    fn from(variant: Vp1NbLineBuffer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vp1NbLineBuffer {
    type Ux = u8;
}
impl crate::IsEnum for Vp1NbLineBuffer {}
#[doc = "Field `VP1_NB_LINE_BUFFER` reader - 17:16\\]
Determines the number of video buffer lines associated to video port #1."]
pub type Vp1NbLineBufferR = crate::FieldReader<Vp1NbLineBuffer>;
impl Vp1NbLineBufferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vp1NbLineBuffer> {
        match self.bits {
            0 => Some(Vp1NbLineBuffer::F0),
            1 => Some(Vp1NbLineBuffer::F1),
            2 => Some(Vp1NbLineBuffer::F2),
            _ => None,
        }
    }
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == Vp1NbLineBuffer::F0
    }
    #[doc = "1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Vp1NbLineBuffer::F1
    }
    #[doc = "2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Vp1NbLineBuffer::F2
    }
}
#[doc = "Field `VP1_NB_LINE_BUFFER` writer - 17:16\\]
Determines the number of video buffer lines associated to video port #1."]
pub type Vp1NbLineBufferW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vp1NbLineBuffer>;
impl<'a, REG> Vp1NbLineBufferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1NbLineBuffer::F0)
    }
    #[doc = "1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1NbLineBuffer::F1)
    }
    #[doc = "2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Vp1NbLineBuffer::F2)
    }
}
#[doc = "20:18\\]
Determines the video line buffer size associated to video port #2 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vp2LineBufferSize {
    #[doc = "1: 512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    F1 = 1,
    #[doc = "2: 682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    F2 = 2,
    #[doc = "3: 853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    F3 = 3,
    #[doc = "4: 1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    F4 = 4,
    #[doc = "5: 1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    F5 = 5,
    #[doc = "6: 1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    F6 = 6,
}
impl From<Vp2LineBufferSize> for u8 {
    #[inline(always)]
    fn from(variant: Vp2LineBufferSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vp2LineBufferSize {
    type Ux = u8;
}
impl crate::IsEnum for Vp2LineBufferSize {}
#[doc = "Field `VP2_LINE_BUFFER_SIZE` reader - 20:18\\]
Determines the video line buffer size associated to video port #2 ."]
pub type Vp2LineBufferSizeR = crate::FieldReader<Vp2LineBufferSize>;
impl Vp2LineBufferSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vp2LineBufferSize> {
        match self.bits {
            1 => Some(Vp2LineBufferSize::F1),
            2 => Some(Vp2LineBufferSize::F2),
            3 => Some(Vp2LineBufferSize::F3),
            4 => Some(Vp2LineBufferSize::F4),
            5 => Some(Vp2LineBufferSize::F5),
            6 => Some(Vp2LineBufferSize::F6),
            _ => None,
        }
    }
    #[doc = "512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Vp2LineBufferSize::F1
    }
    #[doc = "682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Vp2LineBufferSize::F2
    }
    #[doc = "853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    #[inline(always)]
    pub fn is_f3(&self) -> bool {
        *self == Vp2LineBufferSize::F3
    }
    #[doc = "1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    #[inline(always)]
    pub fn is_f4(&self) -> bool {
        *self == Vp2LineBufferSize::F4
    }
    #[doc = "1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    #[inline(always)]
    pub fn is_f5(&self) -> bool {
        *self == Vp2LineBufferSize::F5
    }
    #[doc = "1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    #[inline(always)]
    pub fn is_f6(&self) -> bool {
        *self == Vp2LineBufferSize::F6
    }
}
#[doc = "Field `VP2_LINE_BUFFER_SIZE` writer - 20:18\\]
Determines the video line buffer size associated to video port #2 ."]
pub type Vp2LineBufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vp2LineBufferSize>;
impl<'a, REG> Vp2LineBufferSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "512x24-bits, 682x18-bits, 768x16bits (memory of 384x32-bits)"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F1)
    }
    #[doc = "682x24-bits, 910x18-bits, 1024x16bits (memory of 512x32-bits)"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F2)
    }
    #[doc = "853x24-bits, 1137x18-bits, 1280x16bits (memory of 640x32-bits)"]
    #[inline(always)]
    pub fn f3(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F3)
    }
    #[doc = "1024x24-bits, 1365x18-bits, 1536x16bits (memory of 768x32-bits)"]
    #[inline(always)]
    pub fn f4(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F4)
    }
    #[doc = "1194x24-bits, 1592x18-bits, 1792x16bits (memory of 896x32-bits)"]
    #[inline(always)]
    pub fn f5(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F5)
    }
    #[doc = "1365x24-bits, 1820x18-bits, 2048x16bits (memory of 1024x32-bits)"]
    #[inline(always)]
    pub fn f6(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2LineBufferSize::F6)
    }
}
#[doc = "Field `RESERVED2` reader - 21:21\\]
Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 21:21\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "23:22\\]
Determines the number of video buffer lines associated to video port #2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vp2NbLineBuffer {
    #[doc = "0: No line buffer"]
    F0 = 0,
    #[doc = "1: 1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    F1 = 1,
    #[doc = "2: 2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    F2 = 2,
}
impl From<Vp2NbLineBuffer> for u8 {
    #[inline(always)]
    fn from(variant: Vp2NbLineBuffer) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vp2NbLineBuffer {
    type Ux = u8;
}
impl crate::IsEnum for Vp2NbLineBuffer {}
#[doc = "Field `VP2_NB_LINE_BUFFER` reader - 23:22\\]
Determines the number of video buffer lines associated to video port #2."]
pub type Vp2NbLineBufferR = crate::FieldReader<Vp2NbLineBuffer>;
impl Vp2NbLineBufferR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vp2NbLineBuffer> {
        match self.bits {
            0 => Some(Vp2NbLineBuffer::F0),
            1 => Some(Vp2NbLineBuffer::F1),
            2 => Some(Vp2NbLineBuffer::F2),
            _ => None,
        }
    }
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn is_f0(&self) -> bool {
        *self == Vp2NbLineBuffer::F0
    }
    #[doc = "1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn is_f1(&self) -> bool {
        *self == Vp2NbLineBuffer::F1
    }
    #[doc = "2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn is_f2(&self) -> bool {
        *self == Vp2NbLineBuffer::F2
    }
}
#[doc = "Field `VP2_NB_LINE_BUFFER` writer - 23:22\\]
Determines the number of video buffer lines associated to video port #2."]
pub type Vp2NbLineBufferW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vp2NbLineBuffer>;
impl<'a, REG> Vp2NbLineBufferW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No line buffer"]
    #[inline(always)]
    pub fn f0(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2NbLineBuffer::F0)
    }
    #[doc = "1 line buffer of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn f1(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2NbLineBuffer::F1)
    }
    #[doc = "2 line buffers of the size defined in LINE_BUFFER_SIZE"]
    #[inline(always)]
    pub fn f2(self) -> &'a mut crate::W<REG> {
        self.variant(Vp2NbLineBuffer::F2)
    }
}
#[doc = "24:24\\]
Number of video ports\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NbVideoPorts {
    #[doc = "0: Video port 1 only is present"]
    SingleVp = 0,
    #[doc = "1: Video port 1 and video port 2 are present"]
    DualVp = 1,
}
impl From<NbVideoPorts> for bool {
    #[inline(always)]
    fn from(variant: NbVideoPorts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB_VIDEO_PORTS` reader - 24:24\\]
Number of video ports"]
pub type NbVideoPortsR = crate::BitReader<NbVideoPorts>;
impl NbVideoPortsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NbVideoPorts {
        match self.bits {
            false => NbVideoPorts::SingleVp,
            true => NbVideoPorts::DualVp,
        }
    }
    #[doc = "Video port 1 only is present"]
    #[inline(always)]
    pub fn is_single_vp(&self) -> bool {
        *self == NbVideoPorts::SingleVp
    }
    #[doc = "Video port 1 and video port 2 are present"]
    #[inline(always)]
    pub fn is_dual_vp(&self) -> bool {
        *self == NbVideoPorts::DualVp
    }
}
#[doc = "Field `NB_VIDEO_PORTS` writer - 24:24\\]
Number of video ports"]
pub type NbVideoPortsW<'a, REG> = crate::BitWriter<'a, REG, NbVideoPorts>;
impl<'a, REG> NbVideoPortsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Video port 1 only is present"]
    #[inline(always)]
    pub fn single_vp(self) -> &'a mut crate::W<REG> {
        self.variant(NbVideoPorts::SingleVp)
    }
    #[doc = "Video port 1 and video port 2 are present"]
    #[inline(always)]
    pub fn dual_vp(self) -> &'a mut crate::W<REG> {
        self.variant(NbVideoPorts::DualVp)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Determines the data TX FIFO depth (33-bit words) on the slave port."]
    #[inline(always)]
    pub fn tx_fifodepth(&self) -> TxFifodepthR {
        TxFifodepthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Determines the data RX FIFO depth (32-bit words) on the slave port."]
    #[inline(always)]
    pub fn rx_fifodepth(&self) -> RxFifodepthR {
        RxFifodepthR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Determines the number of DMA_REQ signals."]
    #[inline(always)]
    pub fn nb_dma_request(&self) -> NbDmaRequestR {
        NbDmaRequestR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Determines the number of data lanes supported by the CSI2 protocol engine ."]
    #[inline(always)]
    pub fn nb_data_lanes(&self) -> NbDataLanesR {
        NbDataLanesR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Determines the video line buffer size associated to video port #1 ."]
    #[inline(always)]
    pub fn vp1_line_buffer_size(&self) -> Vp1LineBufferSizeR {
        Vp1LineBufferSizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines the number of video buffer lines associated to video port #1."]
    #[inline(always)]
    pub fn vp1_nb_line_buffer(&self) -> Vp1NbLineBufferR {
        Vp1NbLineBufferR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Determines the video line buffer size associated to video port #2 ."]
    #[inline(always)]
    pub fn vp2_line_buffer_size(&self) -> Vp2LineBufferSizeR {
        Vp2LineBufferSizeR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Determines the number of video buffer lines associated to video port #2."]
    #[inline(always)]
    pub fn vp2_nb_line_buffer(&self) -> Vp2NbLineBufferR {
        Vp2NbLineBufferR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Number of video ports"]
    #[inline(always)]
    pub fn nb_video_ports(&self) -> NbVideoPortsR {
        NbVideoPortsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Determines the data TX FIFO depth (33-bit words) on the slave port."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifodepth(&mut self) -> TxFifodepthW<Csi2GnqSpec> {
        TxFifodepthW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Determines the data RX FIFO depth (32-bit words) on the slave port."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifodepth(&mut self) -> RxFifodepthW<Csi2GnqSpec> {
        RxFifodepthW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Determines the number of DMA_REQ signals."]
    #[inline(always)]
    #[must_use]
    pub fn nb_dma_request(&mut self) -> NbDmaRequestW<Csi2GnqSpec> {
        NbDmaRequestW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Determines the number of data lanes supported by the CSI2 protocol engine ."]
    #[inline(always)]
    #[must_use]
    pub fn nb_data_lanes(&mut self) -> NbDataLanesW<Csi2GnqSpec> {
        NbDataLanesW::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Determines the video line buffer size associated to video port #1 ."]
    #[inline(always)]
    #[must_use]
    pub fn vp1_line_buffer_size(&mut self) -> Vp1LineBufferSizeW<Csi2GnqSpec> {
        Vp1LineBufferSizeW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Csi2GnqSpec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines the number of video buffer lines associated to video port #1."]
    #[inline(always)]
    #[must_use]
    pub fn vp1_nb_line_buffer(&mut self) -> Vp1NbLineBufferW<Csi2GnqSpec> {
        Vp1NbLineBufferW::new(self, 16)
    }
    #[doc = "Bits 18:20 - 20:18\\]
Determines the video line buffer size associated to video port #2 ."]
    #[inline(always)]
    #[must_use]
    pub fn vp2_line_buffer_size(&mut self) -> Vp2LineBufferSizeW<Csi2GnqSpec> {
        Vp2LineBufferSizeW::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Csi2GnqSpec> {
        Reserved2W::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Determines the number of video buffer lines associated to video port #2."]
    #[inline(always)]
    #[must_use]
    pub fn vp2_nb_line_buffer(&mut self) -> Vp2NbLineBufferW<Csi2GnqSpec> {
        Vp2NbLineBufferW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Number of video ports"]
    #[inline(always)]
    #[must_use]
    pub fn nb_video_ports(&mut self) -> NbVideoPortsW<Csi2GnqSpec> {
        NbVideoPortsW::new(self, 24)
    }
}
#[doc = "GENERIC PARAMETER REGISTER This register provide a way to read the generic parameters used in the design.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_gnq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_gnq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2GnqSpec;
impl crate::RegisterSpec for Csi2GnqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_gnq::R`](R) reader structure"]
impl crate::Readable for Csi2GnqSpec {}
#[doc = "`write(|w| ..)` method takes [`csi2_gnq::W`](W) writer structure"]
impl crate::Writable for Csi2GnqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_GNQ to value 0"]
impl crate::Resettable for Csi2GnqSpec {
    const RESET_VALUE: u32 = 0;
}
