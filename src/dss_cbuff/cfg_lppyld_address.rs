#[doc = "Register `CFG_LPPYLD_ADDRESS` reader"]
pub type R = crate::R<CfgLppyldAddressSpec>;
#[doc = "Register `CFG_LPPYLD_ADDRESS` writer"]
pub type W = crate::W<CfgLppyldAddressSpec>;
#[doc = "Field `CFG_LPPYLD_ADDRESS` reader - 31:0\\]
CSI2 only Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_PAYLOAD Address in the CSI Protocol Engine"]
pub type CfgLppyldAddressR = crate::FieldReader<u32>;
#[doc = "Field `CFG_LPPYLD_ADDRESS` writer - 31:0\\]
CSI2 only Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_PAYLOAD Address in the CSI Protocol Engine"]
pub type CfgLppyldAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 only Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_PAYLOAD Address in the CSI Protocol Engine"]
    #[inline(always)]
    pub fn cfg_lppyld_address(&self) -> CfgLppyldAddressR {
        CfgLppyldAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI2 only Programming : Configure the CSI_PROTOCOL_ENGINE__CSI_VC_LONG_PACKET_PAYLOAD Address in the CSI Protocol Engine"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_lppyld_address(&mut self) -> CfgLppyldAddressW<CfgLppyldAddressSpec> {
        CfgLppyldAddressW::new(self, 0)
    }
}
#[doc = "Long payload Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_lppyld_address::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_lppyld_address::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgLppyldAddressSpec;
impl crate::RegisterSpec for CfgLppyldAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_lppyld_address::R`](R) reader structure"]
impl crate::Readable for CfgLppyldAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_lppyld_address::W`](W) writer structure"]
impl crate::Writable for CfgLppyldAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_LPPYLD_ADDRESS to value 0"]
impl crate::Resettable for CfgLppyldAddressSpec {
    const RESET_VALUE: u32 = 0;
}
