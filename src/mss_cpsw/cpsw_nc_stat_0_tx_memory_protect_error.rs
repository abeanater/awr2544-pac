#[doc = "Register `CPSW_NC_STAT_0_TX_MEMORY_PROTECT_ERROR` reader"]
pub type R = crate::R<CpswNcStat0TxMemoryProtectErrorSpec>;
#[doc = "Register `CPSW_NC_STAT_0_TX_MEMORY_PROTECT_ERROR` writer"]
pub type W = crate::W<CpswNcStat0TxMemoryProtectErrorSpec>;
#[doc = "Field `TRANSMIT_MEMORY_PROTECT` reader - 7:0\\]
Transmit Memory Protect CRC Error"]
pub type TransmitMemoryProtectR = crate::FieldReader;
#[doc = "Field `TRANSMIT_MEMORY_PROTECT` writer - 7:0\\]
Transmit Memory Protect CRC Error"]
pub type TransmitMemoryProtectW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Memory Protect CRC Error"]
    #[inline(always)]
    pub fn transmit_memory_protect(&self) -> TransmitMemoryProtectR {
        TransmitMemoryProtectR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit Memory Protect CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_memory_protect(
        &mut self,
    ) -> TransmitMemoryProtectW<CpswNcStat0TxMemoryProtectErrorSpec> {
        TransmitMemoryProtectW::new(self, 0)
    }
}
#[doc = "Transmit Memory Protect CRC Error\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_tx_memory_protect_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_tx_memory_protect_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0TxMemoryProtectErrorSpec;
impl crate::RegisterSpec for CpswNcStat0TxMemoryProtectErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_tx_memory_protect_error::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0TxMemoryProtectErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_tx_memory_protect_error::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0TxMemoryProtectErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_TX_MEMORY_PROTECT_ERROR to value 0"]
impl crate::Resettable for CpswNcStat0TxMemoryProtectErrorSpec {
    const RESET_VALUE: u32 = 0;
}
