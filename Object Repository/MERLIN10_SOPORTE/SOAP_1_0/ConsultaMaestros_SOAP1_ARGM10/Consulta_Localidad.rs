<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Consulta_Localidad</name>
   <tag></tag>
   <elementGuidId>d8d0ca84-4fe5-4f31-aef1-7d2bf5411c4e</elementGuidId>
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
      &lt;soap:queryN4>
         &lt;!--Optional:-->
         &lt;nivel1>AR&lt;/nivel1>
         &lt;!--Optional:-->
         &lt;nivel2>buenos aires&lt;/nivel2>
         &lt;!--Optional:-->
         &lt;nivel4>trenque lauqun&lt;/nivel4>
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
      &lt;/soap:queryN4>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryCalle</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.ConsultaMaestroSoap</defaultValue>
      <description></description>
      <id>957a4a5b-7928-4acd-bb70-b1104185bcc6</id>
      <masked>false</masked>
      <name>ConsultaMaestroSoap</name>
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



WS.verifyElementText(response, 'queryN4Response.return.desc', '')
WS.verifyElementText(response, 'queryN4Response.return.id', '54110403005227')
WS.verifyElementText(response, 'queryN4Response.return.name', 'TRENQUE LAUQUEN')
WS.verifyElementText(response, 'queryN4Response.return.relieved', 'true')
WS.verifyElementText(response, 'queryN4Response.return.synonymous', '')
WS.verifyElementText(response, 'queryN4Response.return.x', '-62.732085')
WS.verifyElementText(response, 'queryN4Response.return.y', '-35.976745')</verificationScript>
   <wsdlAddress>${ConsultaMaestroSoap}</wsdlAddress>
</WebServiceRequestEntity>
