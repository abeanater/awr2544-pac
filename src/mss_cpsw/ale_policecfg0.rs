#[doc = "Register `ALE_POLICECFG0` reader"]
pub type R = crate::R<AlePolicecfg0Spec>;
#[doc = "Register `ALE_POLICECFG0` writer"]
pub type W = crate::W<AlePolicecfg0Spec>;
#[doc = "Field `OUI_TABLE_ENTRY` reader - 4:0\\]
OUI Table Entry Index - Specifies the ALE ONU address lookup table index to match for the selected policing/classifier entry"]
pub type OuiTableEntryR = crate::FieldReader;
#[doc = "Field `OUI_TABLE_ENTRY` writer - 4:0\\]
OUI Table Entry Index - Specifies the ALE ONU address lookup table index to match for the selected policing/classifier entry"]
pub type OuiTableEntryW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUI_MATCH_ENABLE` reader - 15:15\\]
OUI Match Enable - Enables frame ONU address match for the selected policing/classifier entry"]
pub type OuiMatchEnableR = crate::BitReader;
#[doc = "Field `OUI_MATCH_ENABLE` writer - 15:15\\]
OUI Match Enable - Enables frame ONU address match for the selected policing/classifier entry"]
pub type OuiMatchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIORITY_VALUE_SPECIFIES` reader - 18:16\\]
Priority Value - Specifies the frame priority to match for the selected policing/classifier entry"]
pub type PriorityValueSpecifiesR = crate::FieldReader;
#[doc = "Field `PRIORITY_VALUE_SPECIFIES` writer - 18:16\\]
Priority Value - Specifies the frame priority to match for the selected policing/classifier entry"]
pub type PriorityValueSpecifiesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_MATCH_ENABLE` reader - 19:19\\]
Priority Match Enable - Enables frame priority match for the selected policing/classifier entry"]
pub type PriorityMatchEnableR = crate::BitReader;
#[doc = "Field `PRIORITY_MATCH_ENABLE` writer - 19:19\\]
Priority Match Enable - Enables frame priority match for the selected policing/classifier entry"]
pub type PriorityMatchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_NUMBER_SPECIFIES` reader - 25:25\\]
Port Number - Specifies the port address to match for the selected policing/classifier entry"]
pub type PortNumberSpecifiesR = crate::BitReader;
#[doc = "Field `PORT_NUMBER_SPECIFIES` writer - 25:25\\]
Port Number - Specifies the port address to match for the selected policing/classifier entry"]
pub type PortNumberSpecifiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ID_WHEN` reader - 30:30\\]
Trunk ID - When set indicates the port number is a trunk group."]
pub type TrunkIdWhenR = crate::BitReader;
#[doc = "Field `TRUNK_ID_WHEN` writer - 30:30\\]
Trunk ID - When set indicates the port number is a trunk group."]
pub type TrunkIdWhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_MATCH_ENABLE` reader - 31:31\\]
Port Match Enable - Enabled port match for the selected policing/classifier entry"]
pub type PortMatchEnableR = crate::BitReader;
#[doc = "Field `PORT_MATCH_ENABLE` writer - 31:31\\]
Port Match Enable - Enabled port match for the selected policing/classifier entry"]
pub type PortMatchEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
OUI Table Entry Index - Specifies the ALE ONU address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn oui_table_entry(&self) -> OuiTableEntryR {
        OuiTableEntryR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
OUI Match Enable - Enables frame ONU address match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn oui_match_enable(&self) -> OuiMatchEnableR {
        OuiMatchEnableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Priority Value - Specifies the frame priority to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn priority_value_specifies(&self) -> PriorityValueSpecifiesR {
        PriorityValueSpecifiesR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
Priority Match Enable - Enables frame priority match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn priority_match_enable(&self) -> PriorityMatchEnableR {
        PriorityMatchEnableR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Port Number - Specifies the port address to match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn port_number_specifies(&self) -> PortNumberSpecifiesR {
        PortNumberSpecifiesR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Trunk ID - When set indicates the port number is a trunk group."]
    #[inline(always)]
    pub fn trunk_id_when(&self) -> TrunkIdWhenR {
        TrunkIdWhenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Port Match Enable - Enabled port match for the selected policing/classifier entry"]
    #[inline(always)]
    pub fn port_match_enable(&self) -> PortMatchEnableR {
        PortMatchEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
OUI Table Entry Index - Specifies the ALE ONU address lookup table index to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn oui_table_entry(&mut self) -> OuiTableEntryW<AlePolicecfg0Spec> {
        OuiTableEntryW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
OUI Match Enable - Enables frame ONU address match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn oui_match_enable(&mut self) -> OuiMatchEnableW<AlePolicecfg0Spec> {
        OuiMatchEnableW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Priority Value - Specifies the frame priority to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn priority_value_specifies(&mut self) -> PriorityValueSpecifiesW<AlePolicecfg0Spec> {
        PriorityValueSpecifiesW::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
Priority Match Enable - Enables frame priority match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn priority_match_enable(&mut self) -> PriorityMatchEnableW<AlePolicecfg0Spec> {
        PriorityMatchEnableW::new(self, 19)
    }
    #[doc = "Bit 25 - 25:25\\]
Port Number - Specifies the port address to match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn port_number_specifies(&mut self) -> PortNumberSpecifiesW<AlePolicecfg0Spec> {
        PortNumberSpecifiesW::new(self, 25)
    }
    #[doc = "Bit 30 - 30:30\\]
Trunk ID - When set indicates the port number is a trunk group."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_id_when(&mut self) -> TrunkIdWhenW<AlePolicecfg0Spec> {
        TrunkIdWhenW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Port Match Enable - Enabled port match for the selected policing/classifier entry"]
    #[inline(always)]
    #[must_use]
    pub fn port_match_enable(&mut self) -> PortMatchEnableW<AlePolicecfg0Spec> {
        PortMatchEnableW::new(self, 31)
    }
}
#[doc = "The Policing Config 0 holds the port, frame priority and ONU address index as well as match enables for port, frame priority and ONU address matching.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_policecfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_policecfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlePolicecfg0Spec;
impl crate::RegisterSpec for AlePolicecfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_policecfg0::R`](R) reader structure"]
impl crate::Readable for AlePolicecfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_policecfg0::W`](W) writer structure"]
impl crate::Writable for AlePolicecfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_POLICECFG0 to value 0"]
impl crate::Resettable for AlePolicecfg0Spec {
    const RESET_VALUE: u32 = 0;
}
