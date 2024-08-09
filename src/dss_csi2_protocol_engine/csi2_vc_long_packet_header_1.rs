#[doc = "Register `CSI2_VC_LONG_PACKET_HEADER_1` reader"]
pub type R = crate::R<Csi2VcLongPacketHeader1Spec>;
#[doc = "Register `CSI2_VC_LONG_PACKET_HEADER_1` writer"]
pub type W = crate::W<Csi2VcLongPacketHeader1Spec>;
#[doc = "Field `HEADER` reader - 31:0\\]
Packet header information: DATA ID + DATA FIELD +ECC"]
pub type HeaderR = crate::FieldReader<u32>;
#[doc = "Field `HEADER` writer - 31:0\\]
Packet header information: DATA ID + DATA FIELD +ECC"]
pub type HeaderW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Packet header information: DATA ID + DATA FIELD +ECC"]
    #[inline(always)]
    pub fn header(&self) -> HeaderR {
        HeaderR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Packet header information: DATA ID + DATA FIELD +ECC"]
    #[inline(always)]
    #[must_use]
    pub fn header(&mut self) -> HeaderW<Csi2VcLongPacketHeader1Spec> {
        HeaderW::new(self, 0)
    }
}
#[doc = "LONG PACKET HEADER INFORMATION -Virtual channel This register sets the 32-bit DATA_ID + Word count + ECC (the virtual channel id can be different than VC). The ECC will be computed if ECC_TX_EN is set to 1. DATA_ID is located at bit\\[7:0\\]
WC is located at bit\\[23:8\\]
ECC is located at bit\\[31:24\\]
(Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_header_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_header_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VcLongPacketHeader1Spec;
impl crate::RegisterSpec for Csi2VcLongPacketHeader1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vc_long_packet_header_1::R`](R) reader structure"]
impl crate::Readable for Csi2VcLongPacketHeader1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vc_long_packet_header_1::W`](W) writer structure"]
impl crate::Writable for Csi2VcLongPacketHeader1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VC_LONG_PACKET_HEADER_1 to value 0"]
impl crate::Resettable for Csi2VcLongPacketHeader1Spec {
    const RESET_VALUE: u32 = 0;
}
