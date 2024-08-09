#[doc = "Register `STAT_SAFETY` reader"]
pub type R = crate::R<StatSafetySpec>;
#[doc = "Register `STAT_SAFETY` writer"]
pub type W = crate::W<StatSafetySpec>;
#[doc = "Field `SAF_CRC` reader - 7:0\\]
TRM Desccription : Indicates a CRC error between ADCBuffer and CBUFF. 0 : No Error Non Zero : Error TI Restricted Description : 0 - CRC for col-0 - \\[15:0\\], 1 - CRC for col-1 \\[31:16\\]; 2 - CRC for col-2 \\[47:32\\]; 3 - CRC for col-3 \\[63:48\\]
4 - CRC for col-4 - \\[79:64\\]; 5 - CRC for col-5 \\[95:80\\]; 6 - CRC for col-6 \\[111 :96 ; 7 - for col-7 \\[127:112\\]"]
pub type SafCrcR = crate::FieldReader;
#[doc = "Field `SAF_CRC` writer - 7:0\\]
TRM Desccription : Indicates a CRC error between ADCBuffer and CBUFF. 0 : No Error Non Zero : Error TI Restricted Description : 0 - CRC for col-0 - \\[15:0\\], 1 - CRC for col-1 \\[31:16\\]; 2 - CRC for col-2 \\[47:32\\]; 3 - CRC for col-3 \\[63:48\\]
4 - CRC for col-4 - \\[79:64\\]; 5 - CRC for col-5 \\[95:80\\]; 6 - CRC for col-6 \\[111 :96 ; 7 - for col-7 \\[127:112\\]"]
pub type SafCrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAF_CHIRP_ERR` reader - 8:8\\]
Safety Error. Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
pub type SafChirpErrR = crate::BitReader;
#[doc = "Field `SAF_CHIRP_ERR` writer - 8:8\\]
Safety Error. Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
pub type SafChirpErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF_UNUSED1` reader - 31:9\\]
RESERVED"]
pub type SafUnused1R = crate::FieldReader<u32>;
#[doc = "Field `SAF_UNUSED1` writer - 31:9\\]
RESERVED"]
pub type SafUnused1W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TRM Desccription : Indicates a CRC error between ADCBuffer and CBUFF. 0 : No Error Non Zero : Error TI Restricted Description : 0 - CRC for col-0 - \\[15:0\\], 1 - CRC for col-1 \\[31:16\\]; 2 - CRC for col-2 \\[47:32\\]; 3 - CRC for col-3 \\[63:48\\]
4 - CRC for col-4 - \\[79:64\\]; 5 - CRC for col-5 \\[95:80\\]; 6 - CRC for col-6 \\[111 :96 ; 7 - for col-7 \\[127:112\\]"]
    #[inline(always)]
    pub fn saf_crc(&self) -> SafCrcR {
        SafCrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Safety Error. Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
    #[inline(always)]
    pub fn saf_chirp_err(&self) -> SafChirpErrR {
        SafChirpErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
RESERVED"]
    #[inline(always)]
    pub fn saf_unused1(&self) -> SafUnused1R {
        SafUnused1R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TRM Desccription : Indicates a CRC error between ADCBuffer and CBUFF. 0 : No Error Non Zero : Error TI Restricted Description : 0 - CRC for col-0 - \\[15:0\\], 1 - CRC for col-1 \\[31:16\\]; 2 - CRC for col-2 \\[47:32\\]; 3 - CRC for col-3 \\[63:48\\]
4 - CRC for col-4 - \\[79:64\\]; 5 - CRC for col-5 \\[95:80\\]; 6 - CRC for col-6 \\[111 :96 ; 7 - for col-7 \\[127:112\\]"]
    #[inline(always)]
    #[must_use]
    pub fn saf_crc(&mut self) -> SafCrcW<StatSafetySpec> {
        SafCrcW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Safety Error. Indicates tha the chirpAvailable from ADCBuffer arrived before CBUFF has completed sending out the previous Chirp data."]
    #[inline(always)]
    #[must_use]
    pub fn saf_chirp_err(&mut self) -> SafChirpErrW<StatSafetySpec> {
        SafChirpErrW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn saf_unused1(&mut self) -> SafUnused1W<StatSafetySpec> {
        SafUnused1W::new(self, 9)
    }
}
#[doc = "STAT_SAFETY\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_safety::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat_safety::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSafetySpec;
impl crate::RegisterSpec for StatSafetySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_safety::R`](R) reader structure"]
impl crate::Readable for StatSafetySpec {}
#[doc = "`write(|w| ..)` method takes [`stat_safety::W`](W) writer structure"]
impl crate::Writable for StatSafetySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT_SAFETY to value 0"]
impl crate::Resettable for StatSafetySpec {
    const RESET_VALUE: u32 = 0;
}
