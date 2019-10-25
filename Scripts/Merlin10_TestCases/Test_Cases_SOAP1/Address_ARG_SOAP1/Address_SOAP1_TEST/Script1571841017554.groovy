import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_Barrio_Cerrado'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_EntreCalle30'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_Estandar'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_HSBC'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_MAQ_Telefonica'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_MAQ_Telefonica_Apechea'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_Ruta'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_TN'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_CO_TN_ZonaRiesgo'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_DU_DA'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_DU_DC'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_DU_DZ'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NE_AI'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NE_AI_DTV_N4'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NE_AI_RutaKM_duplicated'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NE_CI'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NE_SinAltura_Geo5'))

WS.sendRequestAndVerify(findTestObject('MERLIN10_SOPORTE/SOAP_1_0/Address_SOAP1_ARGM10/Address_NP_SM'))

