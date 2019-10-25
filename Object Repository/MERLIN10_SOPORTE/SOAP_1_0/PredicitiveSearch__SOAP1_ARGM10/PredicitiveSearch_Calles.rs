<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PredicitiveSearch_Calles</name>
   <tag></tag>
   <elementGuidId>10e51d32-9b8d-4f63-8a35-85d97bce2a31</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap.cartoonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:queryCalle>
         &lt;!--Optional:-->
         &lt;nivel1>AR&lt;/nivel1>
         &lt;!--Optional:-->
         &lt;nivel2>Buenos aires&lt;/nivel2>
         &lt;!--Optional:-->
         &lt;nivel3> &lt;/nivel3>
         &lt;!--Optional:-->
         &lt;nivel4>9 de julio&lt;/nivel4>
         &lt;!--Optional:-->
         &lt;calle>san mar&lt;/calle>
         &lt;!--Optional:-->
         &lt;address>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;sector> &lt;/sector>
            &lt;!--Optional:-->
            &lt;status> &lt;/status>
            &lt;!--Optional:-->
            &lt;userName> &lt;/userName>
         &lt;/address>
         &lt;!--Optional:-->
         &lt;listarTramos> &lt;/listarTramos>
      &lt;/soap:queryCalle>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryCalle</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PredicitiveSearch_SOAP1</defaultValue>
      <description></description>
      <id>2866abb5-d641-4892-a76f-3c0b0b2ba2a5</id>
      <masked>false</masked>
      <name>PredictiveSearch_SOAP1</name>
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



WS.verifyElementText(response, 'queryCalleResponse.return.calleName', 'AVENIDA LIBERTADOR GENERAL SAN MARTIN')
WS.verifyElementText(response, 'queryCalleResponse.return.calleNameShort', 'AVDA LIBERTADOR GRAL SAN MARTIN')
WS.verifyElementText(response, 'queryCalleResponse.return.cp', '6500')
WS.verifyElementText(response, 'queryCalleResponse.return.desde', '601')
WS.verifyElementText(response, 'queryCalleResponse.return.hasta', '4400')
WS.verifyElementText(response, 'queryCalleResponse.return.nivel2', 'BUENOS AIRES')
WS.verifyElementText(response, 'queryCalleResponse.return.nivel3', '9 DE JULIO')
WS.verifyElementText(response, 'queryCalleResponse.return.nivel4', '9 DE JULIO')
WS.verifyElementText(response, 'queryCalleResponse.return.x', '-60.879058')
WS.verifyElementText(response, 'queryCalleResponse.return.y', '-35.440261')</verificationScript>
   <wsdlAddress>${PredictiveSearch_SOAP1}</wsdlAddress>
</WebServiceRequestEntity>
