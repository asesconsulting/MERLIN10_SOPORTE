<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CondTrib_NE_Estandar</name>
   <tag></tag>
   <elementGuidId>dad4e72d-2a33-4b09-9a94-f6e99657c429</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.condtribonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:taxStatusNormalize>
         &lt;!--Optional:-->
         &lt;condtribNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapters> &lt;/customAdapters>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xCuit>
               &lt;!--Optional:-->
               &lt;cuit>30546987530&lt;/cuit>
               &lt;!--Optional:-->
               &lt;denomination> &lt;/denomination>
            &lt;/xCuit>
         &lt;/condtribNormalize>
      &lt;/soap:taxStatusNormalize>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>taxStatusNormalize</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Tributario_ARG2</defaultValue>
      <description></description>
      <id>2866abb5-d641-4892-a76f-3c0b0b2ba2a5</id>
      <masked>false</masked>
      <name>Tributario_ARG2</name>
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



WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.status', 'NE')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.cuit', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.denomination', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.denominationFlg', 'SM')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.aernings', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.iva', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.uniqueTribute', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.uniqueTributeActivity', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.employer', '')
WS.verifyElementText(response, 'taxStatusNormalizeResponse.return.nCuit.memberOfSociety', '')</verificationScript>
   <wsdlAddress>${Tributario_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
