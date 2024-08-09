#[doc = "Register `CSI2_VC_LONG_PACKET_PAYLOAD_3` reader"]
pub type R = crate::R<Csi2VcLongPacketPayload3Spec>;
#[doc = "Register `CSI2_VC_LONG_PACKET_PAYLOAD_3` writer"]
pub type W = crate::W<Csi2VcLongPacketPayload3Spec>;
#[doc = "Field `PAYLOAD` reader - 31:0\\]
Packet payload information (excluding check-sum)"]
pub type PayloadR = crate::FieldReader<u32>;
#[doc = "Field `PAYLOAD` writer - 31:0\\]
Packet payload information (excluding check-sum)"]
pub type PayloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Packet payload information (excluding check-sum)"]
    #[inline(always)]
    pub fn payload(&self) -> PayloadR {
        PayloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Packet payload information (excluding check-sum)"]
    #[inline(always)]
    #[must_use]
    pub fn payload(&mut self) -> PayloadW<Csi2VcLongPacketPayload3Spec> {
        PayloadW::new(self, 0)
    }
}
#[doc = "LONG PACKET PAYLOAD INFORMATION -Virtual channel This register sets the payload information (excluding Check-sum). The HW shall capture the word count in the packet header (in CSI2_VC_LONG_PACKET_HEADER) in order to determine the last valid data. (the virtual channel id can be different than VC). Byte1 is bit\\[7:0\\]
Byte2 is bit\\[15:8\\]
Byte3 is bit\\[23:16\\]
Byte4 is bit\\[31:24\\]
Byten is sent before Byten+1 (Least significant byte first and least significant bit first)\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vc_long_packet_payload_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vc_long_packet_payload_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VcLongPacketPayload3Spec;
impl crate::RegisterSpec for Csi2VcLongPacketPayload3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vc_long_packet_payload_3::R`](R) reader structure"]
impl crate::Readable for Csi2VcLongPacketPayload3Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vc_long_packet_payload_3::W`](W) writer structure"]
impl crate::Writable for Csi2VcLongPacketPayload3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VC_LONG_PACKET_PAYLOAD_3 to value 0"]
impl crate::Resettable for Csi2VcLongPacketPayload3Spec {
    const RESET_VALUE: u32 = 0;
}
