#[doc = "Register `CFG_SPHDR_ADDRESS` reader"]
pub type R = crate::R<CfgSphdrAddressSpec>;
#[doc = "Register `CFG_SPHDR_ADDRESS` writer"]
pub type W = crate::W<CfgSphdrAddressSpec>;
#[doc = "Field `CFG_SPHDR_ADDRESS` reader - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_SHORT_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
pub type CfgSphdrAddressR = crate::FieldReader<u32>;
#[doc = "Field `CFG_SPHDR_ADDRESS` writer - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_SHORT_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
pub type CfgSphdrAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_SHORT_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
    #[inline(always)]
    pub fn cfg_sphdr_address(&self) -> CfgSphdrAddressR {
        CfgSphdrAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_SHORT_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sphdr_address(&mut self) -> CfgSphdrAddressW<CfgSphdrAddressSpec> {
        CfgSphdrAddressW::new(self, 0)
    }
}
#[doc = "Short Packet Header Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_sphdr_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_sphdr_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSphdrAddressSpec;
impl crate::RegisterSpec for CfgSphdrAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sphdr_address::R`](R) reader structure"]
impl crate::Readable for CfgSphdrAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_sphdr_address::W`](W) writer structure"]
impl crate::Writable for CfgSphdrAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_SPHDR_ADDRESS to value 0"]
impl crate::Resettable for CfgSphdrAddressSpec {
    const RESET_VALUE: u32 = 0;
}
