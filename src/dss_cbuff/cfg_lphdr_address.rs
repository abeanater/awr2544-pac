#[doc = "Register `CFG_LPHDR_ADDRESS` reader"]
pub type R = crate::R<CfgLphdrAddressSpec>;
#[doc = "Register `CFG_LPHDR_ADDRESS` writer"]
pub type W = crate::W<CfgLphdrAddressSpec>;
#[doc = "Field `CFG_LPHDR_ADDRESS` reader - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
pub type CfgLphdrAddressR = crate::FieldReader<u32>;
#[doc = "Field `CFG_LPHDR_ADDRESS` writer - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
pub type CfgLphdrAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
    #[inline(always)]
    pub fn cfg_lphdr_address(&self) -> CfgLphdrAddressR {
        CfgLphdrAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_HEADER Address in the CSI Protocol Engine LVDS Programming : Configure with the static value : 0x55555555"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lphdr_address(&mut self) -> CfgLphdrAddressW<CfgLphdrAddressSpec> {
        CfgLphdrAddressW::new(self, 0)
    }
}
#[doc = "Long Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lphdr_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lphdr_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLphdrAddressSpec;
impl crate::RegisterSpec for CfgLphdrAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lphdr_address::R`](R) reader structure"]
impl crate::Readable for CfgLphdrAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lphdr_address::W`](W) writer structure"]
impl crate::Writable for CfgLphdrAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LPHDR_ADDRESS to value 0"]
impl crate::Resettable for CfgLphdrAddressSpec {
    const RESET_VALUE: u32 = 0;
}
