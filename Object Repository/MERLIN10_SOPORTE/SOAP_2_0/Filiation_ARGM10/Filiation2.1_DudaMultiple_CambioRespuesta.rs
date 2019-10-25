<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiation2.1_DudaMultiple_CambioRespuesta</name>
   <tag></tag>
   <elementGuidId>97bb003a-e881-4f2a-b9e3-e4dfb4a3bc59</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v1=&quot;http://v1.soap2.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;v1:filiationNormalize2v1>
         &lt;!--Optional:-->
         &lt;filiationNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>3f5c27b150eac642f874a7e4d6f66f75&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xFiliation>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>1000114&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber> &lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>&lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
            &lt;/xFiliation>
         &lt;/filiationNormalize>
      &lt;/v1:filiationNormalize2v1>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>filiationNormalize2v1</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Filiator_ARG_2_1</defaultValue>
      <description></description>
      <id>3cbbeb4b-0c9e-4ae0-96a1-7c2afb91f051</id>
      <masked>false</masked>
      <name>Filiator_ARG_2_1</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementText(response, 'filiationNormalize2v1Response.return.status', 'DU')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.documentType', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.documentTypeFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.documentNumber', '1000114')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.documentNumberFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.tributaryType', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.tributaryTypeFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.tributaryNumber', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.tributaryNumberFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.contributorType', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.contributorTypeFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.lastName', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.lastNameFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.name', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.nameFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.gender', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.genderFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.birthDate', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.birthDateFlg', '')
WS.verifyElementText(response, 'filiationNormalize2v1Response.return.nFiliation.numberAlternativeFiliations', '2')
assertThat(response.getResponseText()).contains('&lt;alternativeFiliations>&lt;documentType>90&lt;/documentType>&lt;documentTypeFlg>AP&lt;/documentTypeFlg>&lt;documentNumber>1000114&lt;/documentNumber>&lt;documentNumberFlg>VA&lt;/documentNumberFlg>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryTypeFlg>AP&lt;/tributaryTypeFlg>&lt;tributaryNumber>20010001146&lt;/tributaryNumber>&lt;tributaryNumberFlg>AP&lt;/tributaryNumberFlg>&lt;contributorType>A&lt;/contributorType>&lt;contributorTypeFlg>AP&lt;/contributorTypeFlg>&lt;lastName>SILVA&lt;/lastName>&lt;lastNameFlg>AP&lt;/lastNameFlg>&lt;name>JORGE&lt;/name>&lt;nameFlg>AP&lt;/nameFlg>&lt;gender>M&lt;/gender>&lt;genderFlg>AP&lt;/genderFlg>&lt;birthDate>01/01/1988&lt;/birthDate>&lt;birthDateFlg>AP&lt;/birthDateFlg>&lt;merlinCustomValues>&lt;merlinCustomValues>&lt;name>differenceLevelName&lt;/name>&lt;value>1&lt;/value>&lt;/merlinCustomValues>&lt;merlinCustomValues>&lt;name>formJuridica&lt;/name>&lt;value>&lt;/value>&lt;/merlinCustomValues>&lt;/merlinCustomValues>&lt;/alternativeFiliations>')
assertThat(response.getResponseText()).contains('&lt;alternativeFiliations>&lt;documentType>96&lt;/documentType>&lt;documentTypeFlg>AP&lt;/documentTypeFlg>&lt;documentNumber>1000114&lt;/documentNumber>&lt;documentNumberFlg>VA&lt;/documentNumberFlg>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryTypeFlg>AP&lt;/tributaryTypeFlg>&lt;tributaryNumber>27010001140&lt;/tributaryNumber>&lt;tributaryNumberFlg>AP&lt;/tributaryNumberFlg>&lt;contributorType>A&lt;/contributorType>&lt;contributorTypeFlg>AP&lt;/contributorTypeFlg>&lt;lastName>ROMA&lt;/lastName>&lt;lastNameFlg>AP&lt;/lastNameFlg>&lt;name>ANGELA ESTHER&lt;/name>&lt;nameFlg>AP&lt;/nameFlg>&lt;gender>F&lt;/gender>&lt;genderFlg>AP&lt;/genderFlg>&lt;birthDate>02/04/1926&lt;/birthDate>&lt;birthDateFlg>AP&lt;/birthDateFlg>&lt;merlinCustomValues>&lt;merlinCustomValues>&lt;name>differenceLevelName&lt;/name>&lt;value>1&lt;/value>&lt;/merlinCustomValues>&lt;merlinCustomValues>&lt;name>formJuridica&lt;/name>&lt;value>&lt;/value>&lt;/merlinCustomValues>&lt;/merlinCustomValues>&lt;/alternativeFiliations>')</verificationScript>
   <wsdlAddress>${Filiator_ARG_2_1}</wsdlAddress>
</WebServiceRequestEntity>
