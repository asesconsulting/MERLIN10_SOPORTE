<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Filiation_NE_SM</name>
   <tag></tag>
   <elementGuidId>e13be179-9841-4cf4-bee6-fd9774be2316</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:filiationNormalize2>
         &lt;!--Optional:-->
         &lt;filiationNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
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
               &lt;documentNumber>65986598&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber> &lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
            &lt;/xFiliation>
         &lt;/filiationNormalize>
      &lt;/soap:filiationNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>filiationNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Filiator_ARG2</defaultValue>
      <description></description>
      <id>3cbbeb4b-0c9e-4ae0-96a1-7c2afb91f051</id>
      <masked>false</masked>
      <name>Filiator_ARG2</name>
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


WS.verifyElementText(response, 'filiationNormalize2Response.return.status', 'NE')
WS.verifyElementText(response, 'filiationNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentType', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentTypeFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumber', '65986598')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.documentNumberFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryType', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryTypeFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumber', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.tributaryNumberFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorType', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.contributorTypeFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastName', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.lastNameFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.name', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.nameFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.gender', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.genderFlg', 'NO')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDate', '')
WS.verifyElementText(response, 'filiationNormalize2Response.return.nFiliation.birthDateFlg', 'NO')</verificationScript>
   <wsdlAddress>${Filiator_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
